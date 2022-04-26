mod channel;
mod prg;
mod util;

mod api;
mod gmw;

pub use crate::channel::ffi::*;
pub use crate::gmw::ffi::*;
pub use crate::prg::ffi::*;
pub use crate::util::ffi::*;
pub mod motion;

/*
pub use channel::{
    ffi::channel
};


pub mod api;
pub mod gmw;
//mod motion;
//pub mod party;
pub mod util;
pub mod channel::ffi;

use libc::c_char;

type Channel = ();

#[no_mangle]
pub extern "C" fn tcp_channel_create_client(addr: *const c_char, port: u16) -> *mut Channel {
    println!("Hello from Rust!");
    Box::into_raw(Box::new(()))
}

#[no_mangle]
pub unsafe extern "C" fn tcp_channel_destroy(raw: *mut Channel) {
    Box::from_raw(raw);
}
*/
