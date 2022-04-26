use crate::channel::*;
use std::cell::RefCell;
use std::net::TcpStream;
use std::rc::Rc;

use std::io::{Read, Write};

use rand::{CryptoRng, Rng};

use crate::*;

pub struct Protocol {
    repr: motion::Backend,
}

impl Protocol {
    pub fn new(my_id: usize, channels: Vec<Channel>) -> Self {
        let mut others_tcp_streams = Vec::with_capacity(channels.len() - 1);
        println!("I am {:?}, {:?}", my_id, channels);

        for (id, channel) in channels.into_iter().enumerate() {
            if id == my_id {
                continue;
            }
            let tcp: Rc<RefCell<TcpStream>> = channel.try_into().expect("TODO");
            println!("{:?}", tcp);
            others_tcp_streams.push(tcp);
        }

        Self {
            repr: motion::Backend::new(my_id, &others_tcp_streams),
        }
    }
}

pub struct Bool {
    repr: motion::Bool,
}

impl Bool {
    pub fn new(protocol: &mut Protocol, share: bool) -> Self {
        Self {
            repr: motion::Bool::new(&mut protocol.repr, share),
        }
    }

    pub fn constant(protocol: &mut Protocol, value: bool) -> Self {
        Self {
            repr: motion::Bool::constant(&mut protocol.repr, value),
        }
    }

    pub fn and(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        Self {
            repr: motion::Bool::and(&mut protocol.repr, &mut a.repr, &mut b.repr),
        }
    }

    pub fn reify(protocol: &mut Protocol, value: &mut Self) -> bool {
        motion::Bool::reify(&mut protocol.repr, &mut value.repr)
    }
}

pub fn share_send_bool<Prg: Rng + CryptoRng, W: Write>(
    prg: &mut Prg,
    channels: &mut [&mut W],
    clear: bool,
) {
    let mut masked = clear;

    for c in channels.iter_mut().skip(1) {
        let share: bool = prg.gen();
        util::write_bool(c, share).expect("TODO");
        masked ^= share;
    }

    util::write_bool(channels[0], masked).expect("TODO")
}

pub fn share_recv_bool<R: Read>(channel: &mut R) -> bool {
    util::read_bool(channel).expect("TODO")
}

pub fn reveal_send_bool<W: Write>(channel: &mut W, share: bool) {
    util::write_bool(channel, share).expect("TODO")
}

pub fn reveal_recv_bool<R: Read>(channels: &mut [&mut R]) -> bool {
    channels.iter_mut().fold(false, |acc, channel| {
        acc ^ util::read_bool(channel).expect("TODO")
    })
}

pub mod ffi {
    use super::*;
    use crate::util::ffi::*;
    use scuttlebutt::AesRng;

    // GMW Protocol

    #[no_mangle]
    pub unsafe extern "C" fn gmw_protocol_new(
        id: usize,
        channels: *const *const Channel,
        channels_len: usize,
    ) -> *mut Protocol {
        let channels = c_to_vec(channels, channels_len)
            .into_iter()
            .map(|channel| (&*channel).clone())
            .collect();
        let ret = Protocol::new(id, channels);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_protocol_drop(protocol: *mut Protocol) {
        Box::from_raw(protocol);
    }

    // GMW Boolean Shares

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_new(protocol: *mut Protocol, share: bool) -> *mut Bool {
        let ret = Bool::new(&mut *protocol, share);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_constant(protocol: *mut Protocol, value: bool) -> *mut Bool {
        let ret = Bool::constant(&mut *protocol, value);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_and(
        protocol: *mut Protocol,
        a: *mut Bool,
        b: *mut Bool,
    ) -> *mut Bool {
        let ret = Bool::and(&mut *protocol, &mut *a, &mut *b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_reify(protocol: *mut Protocol, value: *mut Bool) -> bool {
        Bool::reify(&mut *protocol, &mut *value)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_drop(share: *mut Bool) {
        Box::from_raw(share);
    }

    // GMW Utilities

    #[no_mangle]
    pub unsafe extern "C" fn gmw_share_send_bool(
        prg: *mut AesRng,
        channels: *mut *mut Channel,
        channels_len: usize,
        clear: bool,
    ) {
        let prg = &mut *prg;
        let channels: &mut [&mut Channel] =
            std::mem::transmute(std::slice::from_raw_parts_mut(channels, channels_len));
        share_send_bool(prg, channels, clear)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_share_recv_bool(channel: *mut Channel) -> bool {
        let channel = &mut *channel;
        share_recv_bool(channel)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_reveal_send_bool(channel: *mut Channel, share: bool) {
        let channel = &mut *channel;
        reveal_send_bool(channel, share)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_reveal_recv_bool(
        channels: *mut *mut Channel,
        channels_len: usize,
    ) -> bool {
        let channels: &mut [&mut Channel] =
            std::mem::transmute(std::slice::from_raw_parts_mut(channels, channels_len));
        reveal_recv_bool(channels)
    }
}
