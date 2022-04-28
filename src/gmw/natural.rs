use rand::{CryptoRng, Rng};
use std::io::{Read, Write};

use crate::gmw::Bool;
use crate::gmw::Protocol;
use crate::gmw::*;
use crate::util;
use crate::util::Channel;

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

    pub fn reify(protocol: &mut Protocol, share: &mut Self) -> Vec<u8> {
        let bits: Vec<bool> = share
            .repr
            .iter_mut()
            .map(|b| Bool::reify(protocol, b))
            .collect();
        util::from_bits(&bits)
    }

    pub fn add(protocol: &mut Protocol, a: &mut Self, b: &mut Self) -> Self {
        todo!()
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
    pub unsafe extern "C" fn gmw_nat32_reify(protocol: *mut Protocol, share: *mut Nat) -> u32 {
        u32::from_le_bytes(Nat::reify(&mut *protocol, &mut *share).try_into().unwrap())
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
    pub unsafe extern "C" fn gmw_nat_drop(share: *mut Nat) {
        Box::from_raw(share);
    }
}
