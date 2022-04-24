use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

pub trait Channel: Read + Write {}

impl Channel for TcpStream {}
impl Channel for Cursor<Vec<u8>> {}

pub mod ffi {
    use super::*;
    use libc::c_char;
    use std::ffi::CStr;
    use std::net::Ipv4Addr;
    use std::net::SocketAddr;
    use std::net::TcpListener;
    use std::time::Duration;

    #[no_mangle]
    pub unsafe extern "C" fn channel_destroy(this: *mut Box<dyn Channel>) {
        Box::from_raw(this);
    }

    #[no_mangle]
    pub unsafe extern "C" fn tcp_channel_create_client(
        addr: *const c_char,
        port: u16,
    ) -> *mut Box<dyn Channel> {
        let addr_str = CStr::from_ptr(addr).to_str().unwrap();
        let addr = SocketAddr::from((addr_str.parse::<Ipv4Addr>().unwrap(), port));
        let mut stream = None;
        while stream.is_none() {
            stream = TcpStream::connect(&addr).ok();
        }
        let chan = Box::new(stream.unwrap());
        Box::into_raw(Box::new(chan))
    }

    #[no_mangle]
    pub unsafe extern "C" fn tcp_channel_create_server(
        addr: *const c_char,
        port: u16,
    ) -> *mut Box<dyn Channel> {
        let addr_str = CStr::from_ptr(addr).to_str().unwrap();
        let listener = TcpListener::bind((addr_str, port)).unwrap();
        let stream = listener.accept().unwrap().0;
        let chan = Box::new(stream);
        Box::into_raw(Box::new(chan))
    }

    #[no_mangle]
    pub unsafe extern "C" fn local_channel_create() -> *mut Box<dyn Channel> {
        let buf = Cursor::new(Vec::new());
        let chan = Box::new(buf);
        Box::into_raw(Box::new(chan))
    }
}
