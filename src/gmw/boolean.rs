use crate::gmw::*;
use crate::motion;
use crate::util::Channel;

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

    pub fn reify(protocol: &mut Protocol, value: &mut Self) -> bool {
        motion::Bool::reify(&mut protocol.repr, &mut value.repr)
    }

    pub fn and(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        Self {
            repr: motion::Bool::and(&mut protocol.repr, &mut a.repr, &mut b.repr),
        }
    }
}

pub mod ffi {
    use super::*;
    use scuttlebutt::AesRng;

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
    pub unsafe extern "C" fn gmw_bool_reify(protocol: *mut Protocol, share: *mut Bool) -> bool {
        Bool::reify(&mut *protocol, &mut *share)
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
    pub unsafe extern "C" fn gmw_bool_drop(share: *mut Bool) {
        Box::from_raw(share);
    }

    // Convenience

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
        let buf = [clear as u8; 1];
        share_send(prg, channels, &buf)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_share_recv_bool(channel: *mut Channel) -> bool {
        let channel = &mut *channel;
        let mut buf = [0u8; 1];
        channel.read_exact(&mut buf).expect("TODO");
        buf[0] != 0
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_reveal_send_bool(channel: *mut Channel, share: bool) {
        let channel = &mut *channel;
        let buf = [share as u8; 1];
        channel.write_all(&buf).expect("TODO")
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_reveal_recv_bool(
        channels: *mut *mut Channel,
        channels_len: usize,
    ) -> bool {
        let channels: &mut [&mut Channel] =
            std::mem::transmute(std::slice::from_raw_parts_mut(channels, channels_len));
        let mut buf = [0u8; 1];
        reveal_recv(channels, &mut buf);
        buf[0] != 0
    }
}
