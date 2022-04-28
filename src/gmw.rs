use crate::motion;
use crate::util;
use crate::util::BitVec;
use rand::{CryptoRng, Rng};
use std::io::{Read, Write};

pub struct Protocol {
    repr: motion::Backend,
}

impl Protocol {
    pub fn new(my_id: usize, hosts: Vec<String>, ports: Vec<u16>) -> Self {
        Self {
            repr: motion::Backend::new(my_id, hosts, ports),
        }
    }
}

pub fn share_send<Prg: Rng + CryptoRng, W: Write>(
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

pub fn reveal_recv<R: Read>(channels: &mut [&mut R], clear: &mut [u8]) {
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
}

mod boolean;
pub use boolean::*;

mod natural;
pub use natural::*;

mod integer;
pub use integer::*;
