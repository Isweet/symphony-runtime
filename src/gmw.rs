use crate::motion;
use crate::util;
use crate::util::BitVec;
use rand::{CryptoRng, Rng};
use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;

/// A GMW Protocol instance, each owned by a participating party.
pub struct Protocol {
    my_id: usize,
    hosts: Vec<String>,
    ports: Vec<u16>,
    delayed: Vec<Rc<RefCell<CachedBool>>>,
    party: motion::Party,
}

impl Protocol {
    pub fn new(my_id: usize, hosts: Vec<String>, ports: Vec<u16>) -> Self {
        let party = motion::Party::new(my_id, &hosts, &ports);
        Self {
            my_id,
            hosts,
            ports,
            delayed: Vec::new(),
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

        self.party = motion::Party::new(self.my_id, &self.hosts, &self.ports);
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
pub use natural::Nat;

mod integer;
pub use integer::Int;

#[cfg(test)]
mod tests {
    use super::*;
    use std::*;

    fn protocol_smoke_sized(n: usize) {
        let localhost = "127.0.0.1".to_owned();
        let hosts = vec![localhost; n];
        let ports: Vec<u16> = (0..n).map(|n| (23000 + n) as u16).collect();
        let mut threads = Vec::with_capacity(n);

        let start = time::Instant::now();
        for i in 0..n {
            let h = hosts.clone();
            let p = ports.clone();
            let t = thread::spawn(move || {
                Protocol::new(i, h, p);
            });
            threads.push(t);
        }

        for t in threads {
            t.join().unwrap()
        }
        let elapsed = start.elapsed();
        println!("Elapsed time: {:?}", elapsed);
    }

    #[test]
    fn protocol_smoke() {
        protocol_smoke_sized(10)
    }
}
