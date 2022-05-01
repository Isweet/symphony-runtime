use crate::gmw::*;
use crate::motion;
use crate::util::Channel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum CachedBool {
    Value(bool),
    Expr(motion::Bool),
}

impl CachedBool {
    fn into_expr(self, protocol: &mut Protocol) -> motion::Bool {
        match self {
            CachedBool::Value(share) => motion::Bool::new(&mut protocol.party, share),
            CachedBool::Expr(e) => e,
        }
    }

    fn value(&self, protocol: &mut Protocol) -> Option<bool> {
        match self {
            CachedBool::Value(share) => Some(*share),
            CachedBool::Expr(_) => None,
        }
    }
}

/// A boolean GMW share.
#[derive(Debug, Clone)]
pub struct Bool {
    repr: Rc<RefCell<CachedBool>>,
}

impl Bool {
    pub fn new(protocol: &mut Protocol, share: bool) -> Self {
        let expr = motion::Bool::new(&mut protocol.party, share);
        let repr = Rc::new(RefCell::new(CachedBool::Expr(expr)));
        protocol.delayed.push(repr.clone());
        Self { repr }
    }

    pub fn constant(protocol: &mut Protocol, value: bool) -> Self {
        let expr = motion::Bool::constant(&mut protocol.party, value);
        let repr = Rc::new(RefCell::new(CachedBool::Expr(expr)));
        protocol.delayed.push(repr.clone());
        Self { repr }
    }

    pub fn xor(protocol: &mut Protocol, a: &Self, b: &Self) -> Self {
        let expr_a = (*a.repr).borrow().clone().into_expr(protocol);
        let expr_b = (*b.repr).borrow().clone().into_expr(protocol);
        let expr = expr_a.xor(&expr_b);
        let repr = Rc::new(RefCell::new(CachedBool::Expr(expr)));
        protocol.delayed.push(repr.clone());
        Self { repr }
    }

    pub fn or(protocol: &mut Protocol, a: &Self, b: &Self) -> Self {
        let ab = Bool::and(protocol, a, b);
        let axb = Bool::xor(protocol, a, b);
        Bool::xor(protocol, &ab, &axb)
    }

    pub fn and(protocol: &mut Protocol, a: &Self, b: &Self) -> Self {
        let expr_a = (*a.repr).borrow().clone().into_expr(protocol);
        let expr_b = (*b.repr).borrow().clone().into_expr(protocol);
        let expr = expr_a.and(&expr_b);
        let repr = Rc::new(RefCell::new(CachedBool::Expr(expr)));
        protocol.delayed.push(repr.clone());
        Self { repr }
    }

    pub fn not(protocol: &mut Protocol, share: &Self) -> Self {
        let t = Bool::constant(protocol, true);
        Bool::xor(protocol, share, &t)
    }

    pub fn mux(protocol: &mut Protocol, g: &Self, a: &Self, b: &Self) -> Self {
        let mut tmp = Bool::xor(protocol, a, b);
        tmp = Bool::and(protocol, &tmp, g);
        Bool::xor(protocol, b, &tmp)
    }

    pub fn eq(protocol: &mut Protocol, a: &Self, b: &Self) -> Self {
        let axb = Bool::xor(protocol, a, b);
        Bool::not(protocol, &axb)
    }

    pub fn get(protocol: &mut Protocol, value: &Self) -> bool {
        let cached = (*value.repr).borrow().value(protocol);
        match cached {
            None => {
                protocol.run();
                (*value.repr).borrow().value(protocol).unwrap()
            }
            Some(share) => share,
        }
    }

    pub fn into_raw(this: Self) -> *const RefCell<CachedBool> {
        Rc::into_raw(this.repr)
    }

    pub unsafe fn from_raw(ptr: *const RefCell<CachedBool>) -> Bool {
        Self {
            repr: Rc::from_raw(ptr),
        }
    }
}

pub mod ffi {
    use super::*;
    use scuttlebutt::AesRng;

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_new(
        protocol: *mut Protocol,
        share: bool,
    ) -> *const RefCell<CachedBool> {
        let ret = Bool::new(&mut *protocol, share);
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_constant(
        protocol: *mut Protocol,
        value: bool,
    ) -> *const RefCell<CachedBool> {
        let ret = Bool::constant(&mut *protocol, value);
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_get(
        protocol: *mut Protocol,
        share_raw: *const RefCell<CachedBool>,
    ) -> bool {
        let share = Bool::from_raw(share_raw);
        let ret = Bool::get(&mut *protocol, &share);
        assert_eq!(share_raw, Bool::into_raw(share));
        ret
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_xor(
        protocol: *mut Protocol,
        a_raw: *const RefCell<CachedBool>,
        b_raw: *const RefCell<CachedBool>,
    ) -> *const RefCell<CachedBool> {
        let a = Bool::from_raw(a_raw);
        let b = Bool::from_raw(b_raw);
        let ret = Bool::xor(&mut *protocol, &a, &b);
        assert_eq!(a_raw, Bool::into_raw(a));
        assert_eq!(b_raw, Bool::into_raw(b));
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_and(
        protocol: *mut Protocol,
        a_raw: *const RefCell<CachedBool>,
        b_raw: *const RefCell<CachedBool>,
    ) -> *const RefCell<CachedBool> {
        let a = Bool::from_raw(a_raw);
        let b = Bool::from_raw(b_raw);
        let ret = Bool::and(&mut *protocol, &a, &b);
        assert_eq!(a_raw, Bool::into_raw(a));
        assert_eq!(b_raw, Bool::into_raw(b));
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_drop(share: *const RefCell<CachedBool>) {
        Bool::from_raw(share);
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
