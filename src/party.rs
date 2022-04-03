use std::ffi::CString;
use std::net::SocketAddr;

use crate::motion::*;

pub struct Party {
    config: motion_party_config,
    party: motion_party,
}

unsafe impl Send for Party {}

impl Party {
    pub fn new(my_id: usize, parties: &[SocketAddr]) -> Self {
        let ips: Vec<CString> = parties
            .iter()
            .map(|addr| CString::new(addr.ip().to_string()).unwrap())
            .collect();
        let mut tmp: Vec<party_config> = parties
            .iter()
            .zip(ips.iter())
            .enumerate()
            .map(|(id, (addr, ip))| party_config {
                id: id as size_t,
                host: ip.as_ptr(),
                port: addr.port(),
            })
            .collect();
        let config =
            unsafe { party_config_new(my_id as size_t, tmp.as_mut_ptr(), tmp.len() as size_t) };
        let party = unsafe { party_new(my_id as size_t, config) };
        Self { config, party }
    }

    pub fn my_id(&mut self) -> usize {
        unsafe { party_my_id(self.party) as usize }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let localhost = [127, 0, 0, 1];
        let party0_addr = SocketAddr::from((localhost, 23000));
        let party1_addr = SocketAddr::from((localhost, 23001));
        let parties0 = vec![party0_addr, party1_addr];
        let parties1 = parties0.clone();
        let t0 = std::thread::spawn(move || Party::new(0, &parties0));
        let t1 = std::thread::spawn(move || Party::new(1, &parties1));
        let mut p0 = t0.join().unwrap();
        let mut p1 = t1.join().unwrap();
        assert_eq!(0, p0.my_id());
        assert_eq!(1, p1.my_id());
    }
}
