use std::borrow::Borrow;
use std::cell::RefCell;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::os::unix::prelude::AsRawFd;
use std::rc::Rc;

type LocalStream = Cursor<Vec<u8>>;
type LocalStreamRef = Rc<RefCell<LocalStream>>;
type TcpStreamRef = Rc<RefCell<TcpStream>>;

#[derive(Debug, Clone)]
pub enum Channel {
    Local(LocalStreamRef),
    Tcp(TcpStreamRef),
}

impl Drop for Channel {
    fn drop(&mut self) {
        match self {
            Channel::Local(_) => (),
            Channel::Tcp(tcp) => {
                let x = &*tcp.borrow_mut();
                println!(
                    "Drop TCP: {:?} {:?}",
                    x.as_raw_fd(),
                    Rc::strong_count(&*tcp)
                );
            }
        }
    }
}

impl Channel {
    pub fn new_local() -> Self {
        Channel::Local(Rc::new(RefCell::new(Cursor::new(Vec::new()))))
    }

    pub fn new_tcp(tcp: TcpStream) -> Self {
        Channel::Tcp(Rc::new(RefCell::new(tcp)))
    }
}

impl Read for Channel {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Channel::Local(l) => (*l.borrow_mut()).read(buf),
            Channel::Tcp(s) => (*s.borrow_mut()).read(buf),
        }
    }
}

impl Write for Channel {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Channel::Local(l) => (*l.borrow_mut()).write(buf),
            Channel::Tcp(s) => (*s.borrow_mut()).write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Channel::Local(l) => (*l.borrow_mut()).flush(),
            Channel::Tcp(s) => (*s.borrow_mut()).flush(),
        }
    }
}

impl TryInto<Rc<RefCell<TcpStream>>> for Channel {
    type Error = ();
    fn try_into(self) -> Result<Rc<RefCell<TcpStream>>, Self::Error> {
        match &self {
            Channel::Local(_) => Err(()),
            Channel::Tcp(s) => Ok(s.clone()),
        }
    }
}

pub mod ffi {
    use super::*;
    use libc::c_char;
    use std::ffi::CStr;
    use std::net::TcpListener;

    #[no_mangle]
    pub unsafe extern "C" fn channel_new_local() -> *mut Channel {
        let ret = Channel::new_local();
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn channel_new_tcp_client(
        host: *const c_char,
        port: u16,
    ) -> *mut Channel {
        let host_str = CStr::from_ptr(host).to_str().expect("TODO");
        let mut stream = None;
        while stream.is_none() {
            stream = TcpStream::connect((host_str, port)).ok();
        }
        let ret = Channel::new_tcp(stream.unwrap());
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn channel_new_tcp_server(
        host: *const c_char,
        port: u16,
    ) -> *mut Channel {
        let host_str = CStr::from_ptr(host).to_str().unwrap();
        let listener = TcpListener::bind((host_str, port)).unwrap();
        let stream = listener.accept().expect("TODO").0;
        let ret = Channel::new_tcp(stream);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn channel_drop(this: *mut Channel) {
        Box::from_raw(this);
    }
}
