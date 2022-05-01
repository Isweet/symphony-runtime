use rand::{CryptoRng, Rng};
use std::io::Read;
use std::io::Write;

mod prg;
pub use prg::*;

mod channel;
pub use channel::*;

mod bitvec;
pub use self::bitvec::*;

pub use scuttlebutt::utils::xor_inplace;

pub fn byte_to_bits(mut byte: u8) -> Vec<bool> {
    let mut ret = Vec::with_capacity(u8::BITS as usize);

    for _ in 0..u8::BITS {
        ret.push((byte & 1) == 1);
        byte >>= 1;
    }

    ret
}

pub fn byte_from_bits(bits: &[bool]) -> u8 {
    let mut ret = 0;
    let len = u8::BITS as usize;

    for i in 0..len {
        ret <<= 1;
        ret |= bits[len - i - 1] as u8;
    }

    ret
}

pub fn to_bits(bytes: &[u8]) -> Vec<bool> {
    bytes.iter().flat_map(|b| byte_to_bits(*b)).collect()
}

pub fn from_bits(bits: &[bool]) -> Vec<u8> {
    bits.chunks(8).map(byte_from_bits).collect()
}

mod arith {
    use std::borrow::{Borrow, BorrowMut};

    use crate::gmw::*;

    // SOURCE (lightly modified): https://github.com/emp-toolkit/emp-tool
    pub unsafe fn full_add(
        protocol: &mut Protocol,
        dest: *mut Bool,
        a: *const Bool,
        b: *const Bool,
        size: usize,
    ) {
        if size == 0 {
            return;
        }

        let mut carry = Bool::constant(protocol, false);

        for i in 0..(size - 1) {
            let axc = Bool::xor(protocol, &*a.add(i), &carry);
            let bxc = Bool::xor(protocol, &*b.add(i), &carry);
            *dest.add(i) = Bool::xor(protocol, &*a.add(i), &bxc);
            let t = Bool::and(protocol, &axc, &bxc);
            carry = Bool::xor(protocol, &carry, &t);
        }

        let axb = Bool::xor(protocol, &*a.add(size - 1), &*b.add(size - 1));
        *dest.add(size - 1) = Bool::xor(protocol, &carry, &axb);
    }

    pub unsafe fn full_sub(
        protocol: &mut Protocol,
        dest: *mut Bool,
        borrow_out: *mut Bool,
        a: *const Bool,
        b: *const Bool,
        size: usize,
    ) {
        if size == 0 {
            return;
        }

        let mut borrow = Bool::constant(protocol, false);

        for i in 0..(size - if borrow_out.is_null() { 1 } else { 0 }) {
            let bxa = Bool::xor(protocol, &*a.add(i), &*b.add(i));
            let bxc = Bool::xor(protocol, &borrow, &*b.add(i));
            *dest.add(i) = Bool::xor(protocol, &bxa, &borrow);
            let t = Bool::and(protocol, &bxa, &bxc);
            borrow = Bool::xor(protocol, &borrow, &t);
        }

        if borrow_out.is_null() {
            let bxa = Bool::xor(protocol, &*a.add(size - 1), &*b.add(size - 1));
            *dest.add(size - 1) = Bool::xor(protocol, &bxa, &borrow);
        } else {
            *borrow_out = borrow;
        }
    }

    pub fn full_mul(protocol: &mut Protocol, dest: &mut [Bool], a: &[Bool], b: &[Bool]) {
        debug_assert_eq!(a.len(), b.len());
        debug_assert_eq!(dest.len(), a.len());

        let size = dest.len();
        let mut temp = vec![Bool::constant(protocol, false); size];
        for i in 0..size {
            for j in 0..(size - i) {
                temp[j] = Bool::and(protocol, &a[j], &b[i])
            }
            unsafe {
                full_add(
                    protocol,
                    dest[i..].as_mut_ptr(),
                    dest[i..].as_ptr(),
                    temp.as_ptr(),
                    size - i,
                )
            }
        }
    }

    pub fn full_div(protocol: &mut Protocol, a: &[Bool], b: &[Bool]) -> (Vec<Bool>, Vec<Bool>) {
        let len = a.len();

        let mut overflow = vec![Bool::constant(protocol, false); len];
        overflow[0] = Bool::constant(protocol, false);
        for i in 1..len {
            overflow[i] = Bool::or(protocol, &overflow[i - 1], &b[len - i]);
        }

        let mut temp = vec![Bool::constant(protocol, false); len];
        let mut quot = vec![Bool::constant(protocol, false); len];
        let mut rem = a.to_vec();
        let mut borrow = Bool::constant(protocol, false);

        for i in (0..len).rev() {
            unsafe {
                full_sub(
                    protocol,
                    temp.as_mut_ptr(),
                    &mut borrow as *mut Bool,
                    rem[i..].as_ptr(),
                    b.as_ptr(),
                    len - i,
                );
            }
            borrow = Bool::or(protocol, &borrow, &overflow[i]);
            for j in 0..(len - i) {
                rem[i + j] = Bool::mux(protocol, &borrow, &rem[i + j], &temp[j]);
            }
            quot[i] = Bool::not(protocol, &borrow);
        }

        (quot, rem)
    }

    pub unsafe fn cond_neg(
        protocol: &mut Protocol,
        sign: &Bool,
        dest: *mut Bool,
        src: *const Bool,
        size: usize,
    ) {
        let mut c = sign.clone();

        for i in 0..(size - 1) {
            *dest.add(i) = Bool::xor(protocol, &*src.add(i), sign);
            let t = Bool::xor(protocol, &*dest.add(i), &c);
            c = Bool::and(protocol, &c, &*dest.add(i));
            *dest.add(i) = t;
        }

        let t = Bool::xor(protocol, sign, &c);
        *dest.add(size - 1) = Bool::xor(protocol, &t, &*src.add(size - 1));
    }
}

pub use arith::*;

pub mod ffi {
    pub unsafe fn c_to_vec<T: Clone>(data: *const T, len: usize) -> Vec<T> {
        let mut ret = Vec::with_capacity(len);

        for i in 0..len {
            let element_ref = &*data.add(i);
            ret.push(element_ref.clone())
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_to_bits_sanity() {
        let expected = vec![false, false, true, false, false, false, false, false];
        assert_eq!(byte_to_bits(0x04), expected);
        assert_eq!(byte_from_bits(&byte_to_bits(0x04)), 0x04);
    }

    #[test]
    fn to_bits_sanity() {
        let input: i32 = 1;
        let mut expected = vec![false; 32];
        expected[0] = true;
        assert_eq!(to_bits(&input.to_le_bytes()), expected)
    }
}
