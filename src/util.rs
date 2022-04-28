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

pub fn read_bool<R: Read>(r: &mut R) -> std::io::Result<bool> {
    let mut buf = [0u8];
    r.read_exact(&mut buf).map(|()| buf[0] != 0)
}

pub fn write_bool<W: Write>(w: &mut W, b: bool) -> std::io::Result<()> {
    w.write_all(&[b as u8])
}

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
