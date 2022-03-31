use bitvec::prelude::*;
use rand::CryptoRng;
use rand::Rng;
use std::io::Read;
use std::io::Write;

pub type BV = BitVec;

fn serialize<W: Write>(w: &mut W, bv: &BV) {
    // Turn length into [u8; 8] with usize::to_le_bytes()
    // Turn the `bv` into a Vec<u8> ... somehow
    // Send em over the channel
    todo!();
}

fn deserialize<R: Read>(r: &mut R) -> BV {
    todo!();
}

pub fn send_share<RNG: CryptoRng + Rng, C: Write>(prg: &mut RNG, channels: &mut [C], input: BV) {
    let num_receivers = channels.len();
    let len = input.len();

    let mut masked = input;
    let mut share = bitvec![0; 3];
    for chan in channels.iter_mut().take(num_receivers).skip(1) {
        share.fill_with(|_| prg.gen::<bool>());
        serialize(chan, &share);
        masked ^= &share;
    }

    serialize(&mut channels[0], &masked);
}

pub struct Share(BV);

pub fn recv_share<C: Read>(channel: &mut C) -> Share {
    Share(deserialize(channel))
}

#[cfg(test)]
mod tests {
    use super::*;
    use scuttlebutt::AesRng;

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
        assert_eq!(result[0], expected[0]);
        assert_eq!(result[1], expected[1]);
        assert_eq!(result[2], expected[2]);
    }
}
