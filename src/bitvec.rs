use bincode::deserialize_from;
use bincode::serialize_into;
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use rand::CryptoRng;
use rand::Rng;
use std::io::Read;
use std::io::Write;
use std::ops::*;

pub struct BV(pub BitVec<usize, Lsb0>);

impl BV {
    pub fn zero(len: usize) -> BV {
        let mut ret = BitVec::with_capacity(len);
        for _ in 0..len {
            ret.push(false);
        }
        BV(ret)
    }

    pub fn one(len: usize) -> BV {
        let mut ret = BitVec::with_capacity(len);
        for _ in 0..len {
            ret.push(true);
        }
        BV(ret)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn random<RNG: CryptoRng + Rng>(prg: &mut RNG, length: usize) -> BV {
        let mut ret = BitVec::with_capacity(length);
        for _ in 0..length {
            ret.push(prg.gen());
        }
        BV(ret)
    }

    pub fn send<C: Write>(&self, receiver: &mut C) {
        let bits: Vec<bool> = self.clone().into();
        serialize_into(receiver, &bits).unwrap()
    }

    pub fn recv<C: Read>(sender: &mut C) -> Self {
        let bits: Vec<bool> = deserialize_from(sender).unwrap();
        bits.into()
    }
}

impl Clone for BV {
    fn clone(&self) -> Self {
        BV(self.0.clone())
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

impl From<u32> for BV {
    fn from(item: u32) -> Self {
        let mut ret = bitvec![0; u32::BITS as usize];
        ret.store_le(item);
        BV(ret)
    }
}

impl From<BV> for u32 {
    fn from(item: BV) -> Self {
        item.0.load_le()
    }
}

impl From<Vec<bool>> for BV {
    fn from(item: Vec<bool>) -> Self {
        BV(item.iter().collect())
    }
}

impl From<BV> for Vec<bool> {
    fn from(item: BV) -> Self {
        item.0.iter().by_vals().collect()
    }
}
