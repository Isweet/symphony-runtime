use rand::{CryptoRng, Rng};
use std::io::{Read, Write};

use crate::gmw::Bool;
use crate::gmw::Protocol;
use crate::gmw::*;
use crate::util;
use crate::util::Channel;

#[derive(Clone)]
pub struct Int {
    repr: Vec<Bool>,
}

impl Int {
    pub fn new(protocol: &mut Protocol, share: &[u8]) -> Self {
        let bits = util::to_bits(share);
        Self {
            repr: bits.into_iter().map(|b| Bool::new(protocol, b)).collect(),
        }
    }

    pub fn constant(protocol: &mut Protocol, value: &[u8]) -> Self {
        let bits = util::to_bits(value);
        Self {
            repr: bits
                .into_iter()
                .map(|b| Bool::constant(protocol, b))
                .collect(),
        }
    }

    pub fn xor(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        let repr = a
            .repr
            .iter()
            .zip(b.repr.iter())
            .map(|(a, b)| Bool::xor(protocol, a, b))
            .collect();
        Self { repr }
    }

    pub fn abs(protocol: &mut Protocol, a: &mut Self) -> Self {
        let len = a.repr.len();
        let mut res = Self {
            repr: vec![a.repr[len - 1].clone(); len],
        };
        let mut sum = Int::add(protocol, a, &mut res);

        Int::xor(protocol, &mut sum, &mut res)
    }

    pub fn add(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        debug_assert_eq!(a.repr.len(), b.repr.len());
        let mut repr = vec![Bool::constant(protocol, false); a.repr.len()];
        unsafe {
            util::full_add(
                protocol,
                repr.as_mut_ptr(),
                a.repr.as_ptr(),
                b.repr.as_ptr(),
                repr.len(),
            )
        };
        Self { repr }
    }

    pub fn sub(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        debug_assert_eq!(a.repr.len(), b.repr.len());
        let mut repr = vec![Bool::constant(protocol, false); a.repr.len()];
        unsafe {
            util::full_sub(
                protocol,
                repr.as_mut_ptr(),
                std::ptr::null_mut(),
                a.repr.as_ptr(),
                b.repr.as_ptr(),
                repr.len(),
            )
        };
        Self { repr }
    }

    pub fn mul(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        debug_assert_eq!(a.repr.len(), b.repr.len());
        let mut repr = vec![Bool::constant(protocol, false); a.repr.len()];
        util::full_mul(protocol, &mut repr, &a.repr, &b.repr);
        Self { repr }
    }

    pub fn div(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        debug_assert_eq!(a.repr.len(), b.repr.len());
        let len = a.repr.len();
        let a_abs = Int::abs(protocol, a);
        let b_abs = Int::abs(protocol, b);
        let sign = Bool::xor(protocol, &a.repr[len - 1], &b.repr[len - 1]);
        let (mut repr, _) = util::full_div(protocol, &a_abs.repr, &b_abs.repr);
        unsafe {
            util::cond_neg(protocol, &sign, repr.as_mut_ptr(), repr.as_ptr(), len);
        }
        Self { repr }
    }

    pub fn modulo(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        debug_assert_eq!(a.repr.len(), b.repr.len());
        let len = a.repr.len();
        let a_abs = Int::abs(protocol, a);
        let b_abs = Int::abs(protocol, b);
        let sign = a.repr[len - 1].clone();
        let (_, mut repr) = util::full_div(protocol, &a_abs.repr, &b_abs.repr);
        unsafe {
            util::cond_neg(protocol, &sign, repr.as_mut_ptr(), repr.as_ptr(), len);
        }
        Self { repr }
    }

    pub fn mux(protocol: &mut Protocol, guard: &mut Bool, t: &mut Self, f: &mut Self) -> Self {
        let repr = t
            .repr
            .iter()
            .zip(f.repr.iter())
            .map(|(t, f)| Bool::mux(protocol, guard, t, f))
            .collect();
        Self { repr }
    }

    pub fn eq(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Bool {
        a.repr
            .iter()
            .zip(b.repr.iter())
            .fold(Bool::constant(protocol, true), |acc, (a, b)| {
                let eq = Bool::eq(protocol, a, b);
                Bool::and(protocol, &acc, &eq)
            })
    }

    pub fn gte(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Bool {
        debug_assert_eq!(a.repr.len(), b.repr.len());
        let len = a.repr.len();
        let mut a_ext = a.clone();
        a_ext.repr.push(a_ext.repr[len - 1].clone());

        let mut b_ext = b.clone();
        b_ext.repr.push(b_ext.repr[len - 1].clone());

        let difference = Int::sub(protocol, &mut a_ext, &mut b_ext);
        Bool::not(protocol, &difference.repr[difference.repr.len() - 1])
    }

    pub fn lt(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Bool {
        let tmp = Int::gte(protocol, a, b);
        Bool::not(protocol, &tmp)
    }

    pub fn lte(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Bool {
        Int::gte(protocol, b, a)
    }

    pub fn gt(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Bool {
        let tmp = Int::lte(protocol, a, b);
        Bool::not(protocol, &tmp)
    }

    pub fn get(protocol: &mut Protocol, share: &mut Self) -> Vec<u8> {
        let bits: Vec<bool> = share
            .repr
            .iter_mut()
            .map(|b| Bool::get(protocol, b))
            .collect();
        util::from_bits(&bits)
    }
}

pub mod ffi {
    use super::*;
    use scuttlebutt::AesRng;

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int32_new(protocol: *mut Protocol, share: i32) -> *mut Int {
        let ret = Int::new(&mut *protocol, &share.to_le_bytes());
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int32_constant(protocol: *mut Protocol, value: i32) -> *mut Int {
        let ret = Int::constant(&mut *protocol, &value.to_le_bytes());
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_add(
        protocol: *mut Protocol,
        a: *mut Int,
        b: *mut Int,
    ) -> *mut Int {
        let ret = Int::add(&mut *protocol, &mut *a, &mut *b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_sub(
        protocol: *mut Protocol,
        a: *mut Int,
        b: *mut Int,
    ) -> *mut Int {
        let ret = Int::sub(&mut *protocol, &mut *a, &mut *b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_mul(
        protocol: *mut Protocol,
        a: *mut Int,
        b: *mut Int,
    ) -> *mut Int {
        let ret = Int::mul(&mut *protocol, &mut *a, &mut *b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_div(
        protocol: *mut Protocol,
        a: *mut Int,
        b: *mut Int,
    ) -> *mut Int {
        let ret = Int::div(&mut *protocol, &mut *a, &mut *b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_mod(
        protocol: *mut Protocol,
        a: *mut Int,
        b: *mut Int,
    ) -> *mut Int {
        let ret = Int::modulo(&mut *protocol, &mut *a, &mut *b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_mux(
        protocol: *mut Protocol,
        guard_raw: *const RefCell<CachedBool>,
        t: *mut Int,
        f: *mut Int,
    ) -> *mut Int {
        let mut guard = Bool::from_raw(guard_raw);
        let ret = Int::mux(&mut *protocol, &mut guard, &mut *t, &mut *f);
        Bool::into_raw(guard);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_eq(
        protocol: *mut Protocol,
        a: *mut Int,
        b: *mut Int,
    ) -> *const RefCell<CachedBool> {
        let ret = Int::eq(&mut *protocol, &mut *a, &mut *b);
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_lte(
        protocol: *mut Protocol,
        a: *mut Int,
        b: *mut Int,
    ) -> *const RefCell<CachedBool> {
        let ret = Int::lte(&mut *protocol, &mut *a, &mut *b);
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int32_get(protocol: *mut Protocol, share: *mut Int) -> i32 {
        i32::from_le_bytes(Int::get(&mut *protocol, &mut *share).try_into().unwrap())
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_int_drop(share: *mut Int) {
        Box::from_raw(share);
    }

    // Convenience

    #[no_mangle]
    pub unsafe extern "C" fn gmw_share_send_int32(
        prg: *mut AesRng,
        channels: *mut *mut Channel,
        channels_len: usize,
        clear: i32,
    ) {
        let channels: &mut [&mut Channel] =
            std::mem::transmute(std::slice::from_raw_parts_mut(channels, channels_len));
        share_send(&mut *prg, channels, &clear.to_le_bytes())
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_share_recv_int32(channel: *mut Channel) -> i32 {
        let channel = &mut *channel;
        let mut buf = [0u8; 4];
        channel.read_exact(&mut buf).expect("TODO");
        i32::from_le_bytes(buf)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_reveal_send_int32(channel: *mut Channel, share: i32) {
        let channel = &mut *channel;
        channel.write_all(&share.to_le_bytes()).expect("TODO")
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_reveal_recv_int32(
        channels: *mut *mut Channel,
        channels_len: usize,
    ) -> i32 {
        let channels: &mut [&mut Channel] =
            std::mem::transmute(std::slice::from_raw_parts_mut(channels, channels_len));
        let mut buf = [0u8; 4];
        reveal_recv(channels, &mut buf);
        i32::from_le_bytes(buf)
    }
}
