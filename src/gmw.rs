pub mod eval;

use crate::bitvec::*;

pub trait Share: From<BV> + Into<BV> {}

impl Share for Vec<bool> {}
impl Share for u32 {}

/*
pub type BV = BitVec;

pub fn send_share<RNG: CryptoRng + Rng, C: Write>(prg: &mut RNG, channels: &mut [C], input: BV) {
    let num_receivers = channels.len();
    let len = input.len();

    let mut masked = input;
    let mut share = bitvec![0; 3];
    for chan in channels.iter_mut().take(num_receivers).skip(1) {
        share.fill_with(|_| prg.gen::<bool>());
        serialize_into(chan, &bv_to_vec_bool(&share)).unwrap();
        masked ^= &share;
    }

    serialize_into(&mut channels[0], &bv_to_vec_bool(&masked)).unwrap();
}

pub struct Share(BV);

pub fn recv_share<C: Read>(channel: &mut C) -> Share {
    Share(bv_from_vec_bool(deserialize_from(channel).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use scuttlebutt::AesRng;

    #[test]
    fn serialization_size() {
        let bv = bitvec![0; 8];
        assert_eq!(bincode::serialize(&bv_to_vec_bool(&bv)).unwrap().len(), 16);
    }

    #[test]
    fn send_recv_share() {
        let mut channel1: Vec<u8> = Vec::new();
        let mut channel2: Vec<u8> = Vec::new();
        let mut prg = AesRng::new();
        let expected = bitvec![0; 3];
        send_share(
            &mut prg,
            &mut [&mut channel1, &mut channel2],
            expected.clone(),
        );
        let Share(share1) = recv_share(&mut channel1.as_slice());
        let Share(share2) = recv_share(&mut channel2.as_slice());
        let result = share1 ^ share2;
        assert_eq!(result, expected);
    }
}
*/
