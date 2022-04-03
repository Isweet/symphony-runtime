use bitvec::field::BitField;
use integer_encoding::*;
use rand::Rng;
use std::io::Read;
use std::io::Write;
use std::ops::*;

// TODO(ins): Maybe these should be BitArr instead, no need for dynamic size?
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BitVec(bitvec::vec::BitVec<u8, bitvec::order::Lsb0>);

impl BitVec {
    pub fn new() -> Self {
        Self(bitvec::vec::BitVec::new())
    }

    pub fn with_capacity(len: usize) -> Self {
        Self(bitvec::vec::BitVec::with_capacity(len))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn repeat(bit: bool, len: usize) -> Self {
        Self(bitvec::vec::BitVec::repeat(bit, len))
    }

    pub fn fill(&mut self, value: bool) {
        self.0.fill(value);
    }

    pub fn fill_with<F: FnMut(usize) -> bool>(&mut self, f: F) {
        self.0.fill_with(f);
    }

    pub fn zero(len: usize) -> Self {
        Self::repeat(false, len)
    }

    pub fn one(len: usize) -> Self {
        Self::repeat(true, len)
    }

    pub fn randomize<RNG: Rng>(&mut self, prg: &mut RNG) {
        self.fill_with(|_| prg.gen())
    }

    pub fn random<RNG: Rng>(prg: &mut RNG, len: usize) -> Self {
        let mut ret = Self::zero(len);
        ret.randomize(prg);
        ret
    }

    // TODO(ins): Is there a way to use BitVec<usize> and still write without an extra copy?
    pub fn write<W: Write>(&self, writer: &mut W) {
        writer.write_varint(self.len()).unwrap();
        writer.write_all(self.0.as_raw_slice()).unwrap();
    }

    pub fn read<R: Read>(reader: &mut R) -> Self {
        let len: usize = reader.read_varint().unwrap();
        // TODO(ins): Is there a faster way to compute ceiling div?
        let num_bytes = if len % 8 == 0 { len / 8 } else { len / 8 + 1 };
        let mut bytes = vec![0; num_bytes];
        reader.read_exact(&mut bytes).unwrap();
        let mut ret = Self::new();
        ret.0.extend_from_raw_slice(&bytes);
        ret.0.truncate(len);
        ret
    }
}

impl std::fmt::Display for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<Idx> Index<Idx> for BitVec
where
    bitvec::vec::BitVec<u8, bitvec::order::Lsb0>: Index<Idx>,
{
    type Output = <bitvec::vec::BitVec<u8, bitvec::order::Lsb0> as Index<Idx>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl BitXor for BitVec {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitVec {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&Self> for BitVec {
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.0 ^= &rhs.0;
    }
}

impl From<bool> for BitVec {
    fn from(item: bool) -> Self {
        let mut ret = bitvec::vec::BitVec::with_capacity(1);
        ret.push(item);
        Self(ret)
    }
}

impl From<BitVec> for bool {
    fn from(item: BitVec) -> Self {
        assert_eq!(item.len(), 1);
        item[0]
    }
}

impl From<u32> for BitVec {
    fn from(item: u32) -> Self {
        let mut ret = Self::zero(u32::BITS as usize);
        ret.0.store_le(item);
        ret
    }
}

impl From<BitVec> for u32 {
    fn from(item: BitVec) -> Self {
        assert_eq!(item.len(), u32::BITS as usize);
        item.0.load_le()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_size() {
        let bv = BitVec::zero(8);
        let mut channel = Vec::new();
        bv.write(&mut channel);
        assert_eq!(channel.len(), 2);
        assert_eq!(bv, BitVec::read(&mut channel.as_slice()));
    }

    #[test]
    fn serialize_deserialize() {
        let bv = BitVec::random(&mut rand::thread_rng(), rand::random::<u8>() as usize);
        let mut channel = Vec::new();
        bv.write(&mut channel);
        assert_eq!(BitVec::read(&mut channel.as_slice()), bv);
    }
}
