use rand::CryptoRng;
use rand::Rng;

use std::io::Read;
use std::io::Write;

use crate::util::bitvec::*;

pub type Secure = BitVec;

pub fn send_input<RNG: Rng + CryptoRng, T: Into<Secure>>(
    prg: &mut RNG,
    parties: &mut Vec<Box<dyn Write>>,
    input: T,
) {
    let mut masked = input.into();
    let len = masked.len();

    for party in parties.iter_mut().skip(1) {
        let share = BitVec::random(prg, len);
        share.write(party);
        masked ^= &share;
    }

    masked.write(&mut parties[0]);
}

pub struct GMW {

}

impl GMW {
    pub fn recv_input(&mut self, client: &mut Box<dyn Read>) -> Secure {
        Secure::read(client)
        //        self.shared_in(share)
    }
}

// TODO(ins): Client API is protocol specific
//  but MPC should be able to be generic. Let's try that today.

/*
pub type PartyId = usize;
const PARTY_SIZE: usize = u16::MAX as usize;
pub type GMWShare = BV;

pub struct Party<RNG: Rng + CryptoRng> {
    me: PartyId,
    prg: RNG,
    channels: Vec<Box<dyn Channel>>,
}

impl<RNG: Rng + CryptoRng> Party<RNG> {
    pub fn new(me: PartyId, prg: RNG, channels: Vec<Box<dyn Channel>>) -> Self {
        debug_assert!(channels.len() <= PARTY_SIZE);
        Self { me, prg, channels }
    }

    pub fn num_parties(&self) -> usize {
        self.channels.len()
    }

    fn valid_id(&self, id: PartyId) -> bool {
        id < self.num_parties()
    }

    fn send_input<T: Into<GMWShare>>(&mut self, receiver_ids: Vec<PartyId>, input: T) {
        debug_assert!(receiver_ids
            .iter()
            .all(|receiver_id| self.valid_id(*receiver_id)));
    }

    pub fn send_input_bool(&mut self, receiver_ids: Vec<PartyId>, input: bool) {
        self.send_input(receiver_ids, input);
    }

    pub fn recv_input(&mut self, sender_id: PartyId) -> GMWShare {
        debug_assert!(self.valid_id(sender_id));

        GMWShare::recv(&mut self.channels[sender_id])
    }

    pub fn send_output(&mut self, receiver_id: PartyId, output: GMWShare) {
        debug_assert!(self.valid_id(receiver_id));

        output.send(&mut self.channels[receiver_id])
    }

    fn recv_output<T: From<GMWShare>>(&mut self, sender_ids: Vec<PartyId>) -> T {
        debug_assert!(sender_ids.iter().all(|sender_id| self.valid_id(*sender_id)));

        let mut clear = GMWShare::recv(&mut self.channels[sender_ids[0]]);
        for sender_id in sender_ids.iter().skip(1) {
            clear ^= GMWShare::recv(&mut self.channels[*sender_id]);
        }
        clear.into()
    }

    pub fn recv_output_bool(&mut self, sender_ids: Vec<PartyId>) -> bool {
        self.recv_output(sender_ids)
    }
}
*/
