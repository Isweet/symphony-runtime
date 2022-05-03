use bitvec::macros::internal::funty::Numeric;
use rand::{CryptoRng, Rng};
use std::borrow::Borrow;
use std::io::{Read, Write};

use crate::gmw::Bool;
use crate::gmw::Protocol;
use crate::gmw::*;
use crate::util;
use crate::util::Channel;

use crate::motion;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum CachedNat {
    Value(Vec<bool>),
    Expr(motion::Nat),
}

impl CachedNat {
    fn into_expr(self, protocol: &mut Protocol) -> motion::Nat {
        match self {
            CachedNat::Value(share) => motion::Nat::new(&mut protocol.party, share),
            CachedNat::Expr(e) => e,
        }
    }

    fn value(&self, _protocol: &mut Protocol) -> Option<Vec<bool>> {
        match self {
            CachedNat::Value(share) => Some(share.clone()),
            CachedNat::Expr(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Nat {
    repr: Rc<RefCell<CachedNat>>,
}

impl Nat {
    pub fn from_expr(protocol: &mut Protocol, expr: motion::Nat) -> Self {
        let repr = Rc::new(RefCell::new(CachedNat::Expr(expr)));
        protocol.delayed_nat.push(repr.clone());
        Self { repr }
    }

    pub fn to_expr(protocol: &mut Protocol, share: &Self) -> motion::Nat {
        (*share.repr).borrow().clone().into_expr(protocol)
    }

    pub fn new(protocol: &mut Protocol, share: &[u8]) -> Self {
        let expr = motion::Nat::new(&mut protocol.party, util::to_bits(share));
        Self::from_expr(protocol, expr)
    }

    pub fn constant(protocol: &mut Protocol, value: &[u8]) -> Self {
        let expr = motion::Nat::constant(&mut protocol.party, util::to_bits(value));
        Self::from_expr(protocol, expr)
    }

    pub fn add(protocol: &mut Protocol, a: &Self, b: &Self) -> Self {
        let expr_a = Self::to_expr(protocol, a);
        let expr_b = Self::to_expr(protocol, b);
        let expr = expr_a.add(&expr_b);
        Self::from_expr(protocol, expr)
    }

    pub fn sub(protocol: &mut Protocol, a: &Self, b: &Self) -> Self {
        let expr_a = Self::to_expr(protocol, a);
        let expr_b = Self::to_expr(protocol, b);
        let expr = expr_a.sub(&expr_b);
        Self::from_expr(protocol, expr)
    }

    pub fn mul(protocol: &mut Protocol, a: &Self, b: &Self) -> Self {
        let expr_a = Self::to_expr(protocol, a);
        let expr_b = Self::to_expr(protocol, b);
        let expr = expr_a.mul(&expr_b);
        Self::from_expr(protocol, expr)
    }

    pub fn mux(protocol: &mut Protocol, g: &Bool, a: &Self, b: &Self) -> Self {
        let expr_g = Bool::to_expr(protocol, g);
        let expr_a = Self::to_expr(protocol, a);
        let expr_b = Self::to_expr(protocol, b);
        let expr = motion::Nat::mux(&expr_g, &expr_a, &expr_b);
        Self::from_expr(protocol, expr)
    }

    pub fn eq(protocol: &mut Protocol, a: &Self, b: &Self) -> Bool {
        let expr_a = Self::to_expr(protocol, a);
        let expr_b = Self::to_expr(protocol, b);
        let expr = expr_a.eq(&expr_b);
        Bool::from_expr(protocol, expr)
    }

    pub fn gt(protocol: &mut Protocol, a: &Self, b: &Self) -> Bool {
        let expr_a = Self::to_expr(protocol, a);
        let expr_b = Self::to_expr(protocol, b);
        let expr = expr_a.gt(&expr_b);
        Bool::from_expr(protocol, expr)
    }

    pub fn lt(protocol: &mut Protocol, a: &Self, b: &Self) -> Bool {
        Self::gt(protocol, b, a)
    }

    pub fn gte(protocol: &mut Protocol, a: &Self, b: &Self) -> Bool {
        let altb = Self::lt(protocol, a, b);
        Bool::not(protocol, &altb)
    }

    pub fn lte(protocol: &mut Protocol, a: &Self, b: &Self) -> Bool {
        let agtb = Self::gt(protocol, a, b);
        Bool::not(protocol, &agtb)
    }

    pub fn get(protocol: &mut Protocol, share: &Self) -> Vec<u8> {
        let cached = (*share.repr).borrow().value(protocol);
        let bits = match cached {
            None => {
                protocol.run();
                (*share.repr).borrow().value(protocol).unwrap()
            }
            Some(share) => share,
        };
        util::from_bits(&bits)
    }
}

pub mod ffi {
    use super::*;
    use scuttlebutt::AesRng;

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat32_new(protocol: *mut Protocol, share: u32) -> *mut Nat {
        let ret = Nat::new(&mut *protocol, &share.to_le_bytes());
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat32_constant(protocol: *mut Protocol, value: u32) -> *mut Nat {
        let ret = Nat::constant(&mut *protocol, &value.to_le_bytes());
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_add(
        protocol: *mut Protocol,
        a: *mut Nat,
        b: *mut Nat,
    ) -> *mut Nat {
        let ret = Nat::add(&mut *protocol, &*a, &*b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_mul(
        protocol: *mut Protocol,
        a: *mut Nat,
        b: *mut Nat,
    ) -> *mut Nat {
        let ret = Nat::mul(&mut *protocol, &*a, &*b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_mux(
        protocol: *mut Protocol,
        guard_raw: *const RefCell<CachedBool>,
        t: *mut Nat,
        f: *mut Nat,
    ) -> *mut Nat {
        let mut guard = Bool::from_raw(guard_raw);
        let ret = Nat::mux(&mut *protocol, &guard, &*t, &*f);
        Bool::into_raw(guard);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_eq(
        protocol: *mut Protocol,
        a: *mut Nat,
        b: *mut Nat,
    ) -> *const RefCell<CachedBool> {
        let ret = Nat::eq(&mut *protocol, &*a, &*b);
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_lte(
        protocol: *mut Protocol,
        a: *mut Nat,
        b: *mut Nat,
    ) -> *const RefCell<CachedBool> {
        let ret = Nat::lte(&mut *protocol, &*a, &*b);
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat32_get(protocol: *mut Protocol, share: *mut Nat) -> u32 {
        u32::from_le_bytes(Nat::get(&mut *protocol, &*share).try_into().unwrap())
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_drop(share: *mut Nat) {
        Box::from_raw(share);
    }

    // Convenience

    #[no_mangle]
    pub unsafe extern "C" fn gmw_share_send_nat32(
        prg: *mut AesRng,
        channels: *mut *mut Channel,
        channels_len: usize,
        clear: u32,
    ) {
        let channels: &mut [&mut Channel] =
            std::mem::transmute(std::slice::from_raw_parts_mut(channels, channels_len));
        share_send(&mut *prg, channels, &clear.to_le_bytes())
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_share_recv_nat32(channel: *mut Channel) -> u32 {
        let channel = &mut *channel;
        let mut buf = [0u8; 4];
        channel.read_exact(&mut buf).expect("TODO");
        u32::from_le_bytes(buf)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_reveal_send_nat32(channel: *mut Channel, share: u32) {
        let channel = &mut *channel;
        channel.write_all(&share.to_le_bytes()).expect("TODO")
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_reveal_recv_nat32(
        channels: *mut *mut Channel,
        channels_len: usize,
    ) -> u32 {
        let channels: &mut [&mut Channel] =
            std::mem::transmute(std::slice::from_raw_parts_mut(channels, channels_len));
        let mut buf = [0u8; 4];
        reveal_recv(channels, &mut buf);
        u32::from_le_bytes(buf)
    }
}
