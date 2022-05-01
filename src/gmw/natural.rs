use bitvec::macros::internal::funty::Numeric;
use rand::{CryptoRng, Rng};
use std::io::{Read, Write};

use crate::gmw::Bool;
use crate::gmw::Protocol;
use crate::gmw::*;
use crate::util;
use crate::util::Channel;

#[derive(Clone, Debug)]
pub struct Nat {
    repr: Vec<Bool>,
}

impl Nat {
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
        let mut a_ext = a.clone();
        a_ext.repr.push(Bool::constant(protocol, false));

        let mut b_ext = b.clone();
        b_ext.repr.push(Bool::constant(protocol, false));

        let difference = Nat::sub(protocol, &mut a_ext, &mut b_ext);
        Bool::not(protocol, &difference.repr[difference.repr.len() - 1])
    }

    pub fn lt(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Bool {
        let tmp = Nat::gte(protocol, a, b);
        Bool::not(protocol, &tmp)
    }

    pub fn lte(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Bool {
        Nat::gte(protocol, b, a)
    }

    pub fn gt(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Bool {
        let tmp = Nat::lte(protocol, a, b);
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
        let ret = Nat::add(&mut *protocol, &mut *a, &mut *b);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_mul(
        protocol: *mut Protocol,
        a: *mut Nat,
        b: *mut Nat,
    ) -> *mut Nat {
        let ret = Nat::mul(&mut *protocol, &mut *a, &mut *b);
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
        let ret = Nat::mux(&mut *protocol, &mut guard, &mut *t, &mut *f);
        Bool::into_raw(guard);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_eq(
        protocol: *mut Protocol,
        a: *mut Nat,
        b: *mut Nat,
    ) -> *const RefCell<CachedBool> {
        let ret = Nat::eq(&mut *protocol, &mut *a, &mut *b);
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat_lte(
        protocol: *mut Protocol,
        a: *mut Nat,
        b: *mut Nat,
    ) -> *const RefCell<CachedBool> {
        let ret = Nat::lte(&mut *protocol, &mut *a, &mut *b);
        Bool::into_raw(ret)
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_nat32_get(protocol: *mut Protocol, share: *mut Nat) -> u32 {
        let x = u32::from_le_bytes(Nat::get(&mut *protocol, &mut *share).try_into().unwrap());
        x
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
        let x = u32::from_le_bytes(buf);
        x
    }
}
