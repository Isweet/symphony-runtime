use bitvec::prelude::*;
use bitvec::vec::BitVec;
use rand::CryptoRng;
use rand::Rng;
use std::io::Read;
use std::io::Write;
use std::ops::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BV(BitVec<u8, Lsb0>);

impl BV {
    pub fn new() -> Self {
        Self(BitVec::new())
    }

    pub fn with_capacity(len: usize) -> Self {
        Self(BitVec::with_capacity(len))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn serialize_len(&self) -> [u8; 8] {
        self.0.len().to_le_bytes()
    }

    pub fn repeat(bit: bool, len: usize) -> Self {
        BV(BitVec::repeat(bit, len))
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

    pub fn randomize<RNG: Rng + CryptoRng>(&mut self, prg: &mut RNG) {
        self.fill_with(|_| prg.gen())
    }

    pub fn random<RNG: Rng + CryptoRng>(len: usize, prg: &mut RNG) -> Self {
        let mut ret = Self::zero(len);
        ret.randomize(prg);
        ret
    }

    pub fn serialize_data(&self) -> &[u8] {
        self.0.as_raw_slice()
    }

    pub fn send<C: Write>(&self, receiver: &mut C) {
        receiver.write_all(&self.serialize_len()).unwrap();
        receiver.write_all(self.0.as_raw_slice()).unwrap();
    }

    pub fn recv<C: Read>(sender: &mut C) -> Self {
        let mut len_serialized: [u8; 8] = [0; 8];
        sender.read_exact(&mut len_serialized).unwrap();
        let len = usize::from_le_bytes(len_serialized);
        let mut ret = Self::zero(len);
        sender.read_exact(ret.0.as_raw_mut_slice()).unwrap();
        ret
    }

    pub fn append(&mut self, other: &mut BV) {
        self.0.append(&mut other.0);
    }
}

impl std::fmt::Display for BV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<Idx> Index<Idx> for BV
where
    BitVec<u8, Lsb0>: Index<Idx>,
{
    type Output = <BitVec<u8, Lsb0> as Index<Idx>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl BitXor for BV {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BV {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&Self> for BV {
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.0 ^= &rhs.0;
    }
}

impl From<bool> for BV {
    fn from(item: bool) -> Self {
        let mut ret = BitVec::with_capacity(1);
        ret.push(item);
        BV(ret)
    }
}

impl From<BV> for bool {
    fn from(item: BV) -> Self {
        assert_eq!(item.len(), 1);
        item[0]
    }
}

impl From<u32> for BV {
    fn from(item: u32) -> Self {
        let mut ret = BV::zero(u32::BITS as usize);
        ret.0.store_le(item);
        ret
    }
}

impl From<BV> for u32 {
    fn from(item: BV) -> Self {
        assert_eq!(item.len(), u32::BITS as usize);
        item.0.load_le()
    }
}

// TODO(ins): How to handle vectors?
