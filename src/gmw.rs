use crate::api::*;

pub struct GMW {}

impl Sec<bool> for GMW {
    type Item = bool;
}

pub mod ffi {
    use super::*;
    use crate::channel::Channel;
    use scuttlebutt::AesRng;

    #[no_mangle]
    pub unsafe extern "C" fn gmw_delete(this: *mut GMW) {
        Box::from_raw(this);
    }

    #[no_mangle]
    pub extern "C" fn gmw_create(
        id: usize,
        len_chans: usize,
        chans: *const *mut dyn Channel,
    ) -> *mut GMW {
        todo!()
    }

    type SecBool = <GMW as Sec<bool>>::Item;

    #[no_mangle]
    pub unsafe extern "C" fn gmw_bool_delete(this: *mut SecBool) {
        Box::from_raw(this);
    }

    #[no_mangle]
    pub extern "C" fn gmw_share_send_bool(
        prg: *mut AesRng,
        len_chans: usize,
        chans: *const *mut dyn Channel,
        input: bool,
    ) {
        todo!()
    }

    #[no_mangle]
    pub extern "C" fn gmw_share_recv_bool(this: *mut GMW, chan: *mut dyn Channel) -> *mut SecBool {
        todo!()
    }

    #[no_mangle]
    pub extern "C" fn gmw_reveal_send_bool(
        this: *mut GMW,
        chan: *mut dyn Channel,
        output: *mut SecBool,
    ) {
        todo!()
    }

    #[no_mangle]
    pub extern "C" fn gmw_reveal_recv_bool(
        len_chans: usize,
        chans: *const *mut dyn Channel,
    ) -> bool {
        todo!()
    }

    #[no_mangle]
    pub extern "C" fn gmw_lit_bool(this: *mut GMW, lit: bool) -> *mut SecBool {
        todo!()
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_and_bool(
        this: *mut GMW,
        a: *const SecBool,
        b: *const SecBool,
    ) -> *const SecBool {
        let this_safe = Box::from_raw(this);
        let a_safe = Box::from_raw(a as *mut SecBool);
        let b_safe = Box::from_raw(b as *mut SecBool);
        let ret: Box<bool> = todo!();
        Box::into_raw(this_safe);
        Box::into_raw(a_safe);
        Box::into_raw(b_safe);
        Box::into_raw(ret)
    }
}
/*
use rand::CryptoRng;
use rand::Rng;

use libc::size_t;
use scuttlebutt::AesRng;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::io::Read;
use std::io::Write;
use std::rc::Rc;

impl<Prg: Rng + CryptoRng> SecIO<Prg, bool> for GMW {
    fn send_input(prg: &mut Prg, parties: &mut Vec<Box<dyn Write>>, input: &bool) {
        todo!()
    }

    fn recv_input(&mut self, client: &mut Box<dyn Read>) -> Self::Item {
        todo!()
    }

    fn send_input_secure(prg: &mut Prg, parties: &mut Vec<Box<dyn Write>>, input: &Self::Item) {
        todo!()
    }

    fn recv_input_secure(&mut self, clients: &mut Vec<Box<dyn Read>>) -> Self::Item {
        todo!()
    }

    fn send_output(&mut self, client: &mut Box<dyn Write>, output: &Self::Item) {
        todo!()
    }

    fn recv_output(parties: &mut Vec<Box<dyn Read>>) -> bool {
        todo!()
    }
}

#[no_mangle]
pub unsafe extern "C" fn gmw_send_input_bool(
    prg: *mut AesRng,
    size_parties: libc::size_t,
    parties: *const *mut dyn Write,
    input: &bool,
) {
    let mut parties_vec = Vec::with_capacity(size_parties);
    for i in 0..size_parties {
        let write = *parties.add(i);
        parties_vec.push(Box::from_raw(write));
    }

    GMW::send_input(prg.as_mut().unwrap(), &mut parties_vec, input);

    for w in parties_vec {
        Box::into_raw(w);
    }
}

/*

use std::io::{Read, Write};
use std::rc::Rc;

use once_cell::unsync::Lazy;
use rand::{CryptoRng, Rng};

use crate::api::*;

pub struct GMW {}





#[no_mangle]
pub unsafe extern "C" fn send_input_bool(prg: *mut Prg, parties: , input: &bool) {
    GMW::send_input(Box::from_raw(prg), Box::input)
}

impl GMW {
    pub fn send_input_
}



pub struct GMW {
    backend: Motion,
}

impl GMW {}

impl Sec<bool> for GMW {
    type Item = Rc<Lazy<bool>>;
}

// TODO(ins): SecIO<bool> for GMW

impl SecLit<bool> for GMW {
    fn sec_lit(&mut self, constant: &bool) -> Self::Item {
        let mut g = self.backend.const_bool_gate(*constant);
        Rc::new(Lazy::new(|| g.eval()))
    }
}

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

pub struct GMW {}

impl GMW {
    pub fn recv_input(&mut self, client: &mut Box<dyn Read>) -> Rc<Lazy<Secure>> {
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
*/
*/
