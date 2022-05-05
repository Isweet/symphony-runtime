use crate::motion;
use crate::util;
use crate::util::BitVec;
use rand::{CryptoRng, Rng};
use std::cell::RefCell;
use std::io::{Read, Write};
use std::os::unix::io::RawFd;
use std::os::unix::prelude::FromRawFd;
use std::rc::Rc;

/// A GMW Protocol instance, each owned by a participating party.
pub struct Protocol {
    my_id: usize,
    delayed: Vec<Rc<RefCell<CachedBool>>>,
    delayed_nat: Vec<Rc<RefCell<CachedNat>>>,
    party: motion::Party,
    transports: motion::Transports,
}

impl Protocol {
    pub fn new(my_id: usize, hosts: Vec<String>, ports: Vec<u16>) -> Self {
        let transports = motion::Transports::new(my_id, &hosts, &ports);
        let party = motion::Party::new(my_id, &transports);
        Self {
            my_id,
            delayed: Vec::new(),
            delayed_nat: Vec::new(),
            transports,
            party,
        }
    }

    fn run(&mut self) {
        self.party.run();

        while let Some(cbr) = self.delayed.pop() {
            let r = &mut *cbr.borrow_mut();
            let share = match r {
                CachedBool::Expr(e) => e.get(),
                _ => unreachable!(),
            };
            *r = CachedBool::Value(share);
        }

        while let Some(cnr) = self.delayed_nat.pop() {
            let r = &mut *cnr.borrow_mut();
            let share = match r {
                CachedNat::Expr(e) => e.get(),
                _ => unreachable!(),
            };
            *r = CachedNat::Value(share);
        }

        self.party = motion::Party::new(self.my_id, &self.transports);
    }
}

fn share_send<Prg: Rng + CryptoRng, W: Write>(
    prg: &mut Prg,
    channels: &mut [&mut W],
    clear: &[u8],
) {
    let mut masked = clear.to_vec();
    let mut share = vec![0; masked.len()];

    for c in channels.iter_mut().skip(1) {
        prg.fill_bytes(&mut share);
        c.write_all(&share).expect("TODO");
        util::xor_inplace(&mut masked, &share);
    }

    channels[0].write_all(&masked).expect("TODO")
}

fn reveal_recv<R: Read>(channels: &mut [&mut R], clear: &mut [u8]) {
    clear.iter_mut().for_each(|b| *b = 0);

    let mut share = vec![0u8; clear.len()];

    for c in channels {
        c.read_exact(&mut share).expect("TODO");
        util::xor_inplace(clear, &share);
    }
}

pub mod ffi {
    use super::*;
    use crate::util::ffi::*;
    use std::ffi::CStr;

    #[no_mangle]
    pub unsafe extern "C" fn gmw_protocol_new(
        id: usize,
        hosts: *const *const libc::c_char,
        ports: *const u16,
        len: usize,
    ) -> *mut Protocol {
        let hosts = c_to_vec(hosts, len)
            .into_iter()
            .map(|host_ptr| CStr::from_ptr(host_ptr).to_str().expect("TODO").to_owned())
            .collect();
        let ports = c_to_vec(ports, len);
        let ret = Protocol::new(id, hosts, ports);
        Box::into_raw(Box::new(ret))
    }

    #[no_mangle]
    pub unsafe extern "C" fn gmw_protocol_drop(protocol: *mut Protocol) {
        Box::from_raw(protocol);
    }

    pub use boolean::ffi::*;
    pub use integer::ffi::*;
    pub use natural::ffi::*;
}

mod boolean;
pub use boolean::Bool;
use boolean::CachedBool;

mod natural;
use natural::CachedNat;
pub use natural::Nat;

mod integer;
pub use integer::Int;

#[cfg(test)]
mod tests {}
