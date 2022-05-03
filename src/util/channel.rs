use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::net::TcpStream;
use std::os::unix::io::RawFd;
use std::os::unix::prelude::IntoRawFd;
use std::rc::Rc;

#[derive(Debug)]
pub struct LocalChannel(VecDeque<u8>);

impl LocalChannel {
    pub fn new() -> Self {
        LocalChannel(VecDeque::new())
    }
}

impl Read for LocalChannel {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = buf.len();
        for b in buf {
            *b = self
                .0
                .pop_front()
                .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
        }
        std::io::Result::Ok(size)
    }
}

impl Write for LocalChannel {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for b in buf {
            self.0.push_back(*b)
        }
        std::io::Result::Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::Result::Ok(())
    }
}

pub struct TcpChannel {
    input: BufReader<TcpStream>,
    output: BufWriter<TcpStream>,
}

impl TcpChannel {
    pub fn new(stream: TcpStream) -> Self {
        let input = BufReader::new(stream.try_clone().expect("TODO"));
        let output = BufWriter::new(stream);
        Self { input, output }
    }
}

impl Read for TcpChannel {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.input.read(buf)
    }
}

impl Write for TcpChannel {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.output.flush()
    }
}

pub enum Channel {
    Local(LocalChannel),
    Tcp(TcpChannel),
}

impl Channel {
    pub fn try_get_socket(&self) -> Option<RawFd> {
        match self {
            Channel::Local(_) => None,
            Channel::Tcp(channel) => Some(
                channel
                    .input
                    .get_ref()
                    .try_clone()
                    .expect("TODO")
                    .into_raw_fd(),
            ),
        }
    }
}

impl Read for Channel {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Channel::Local(local) => local.read(buf),
            Channel::Tcp(tcp) => tcp.read(buf),
        }
    }
}

impl Write for Channel {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Channel::Local(local) => local.write(buf),
            Channel::Tcp(tcp) => tcp.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Channel::Local(local) => local.flush(),
            Channel::Tcp(tcp) => tcp.flush(),
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
        let ret = Channel::Local(LocalChannel::new());
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
        let ret = Channel::Tcp(TcpChannel::new(stream.unwrap()));
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
        let ret = Channel::Tcp(TcpChannel::new(stream));
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn channel_drop(this: *mut Channel) {
        Box::from_raw(this);
    }

    #[no_mangle]
    pub unsafe extern "C" fn channel_send_all(this: *mut Channel, buf: *const u8, len: usize) {
        (&mut *this)
            .write_all(std::slice::from_raw_parts(buf, len))
            .expect("TODO")
    }

    #[no_mangle]
    pub unsafe extern "C" fn channel_recv_all(this: *mut Channel, buf: *mut u8, len: usize) {
        (&mut *this)
            .read_exact(std::slice::from_raw_parts_mut(buf, len))
            .expect("TODO")
    }

    #[no_mangle]
    pub unsafe extern "C" fn channel_flush(this: *mut Channel) {
        (&mut *this).flush().expect("TODO")
    }
}
