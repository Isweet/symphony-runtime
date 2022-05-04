use std::os::unix::io::RawFd;

/// A wrapper around the [MOTION](https://github.com/encryptogroup/MOTION) `Party` object.
pub struct Party {
    repr: *mut libc::c_void,
}

impl Party {
    pub fn new(my_id: usize, hosts: &[String], ports: &[u16]) -> Self {
        let c_hosts: Vec<std::ffi::CString> = hosts
            .iter()
            .map(|h| std::ffi::CString::new(h.clone()).expect("TODO"))
            .collect();
        let c_hosts_ptrs: Vec<*const libc::c_char> = c_hosts.iter().map(|h| h.as_ptr()).collect();
        let start = std::time::Instant::now();
        let repr = unsafe {
            ffi::motion_party_new(my_id, c_hosts_ptrs.as_ptr(), ports.as_ptr(), hosts.len())
        };
        println!("Party initialization took {:?}", start.elapsed());
        Self { repr }
    }

    pub fn run(&self) {
        unsafe { ffi::motion_party_run(self.repr) }
    }
}

impl Drop for Party {
    fn drop(&mut self) {
        let x = self.repr;
        unsafe { ffi::motion_party_delete(self.repr) }
    }
}

/// A wrapper around the [MOTION](https://github.com/encryptogroup/MOTION) `ShareWrapper` object, representing a boolean GMW share.
#[derive(Debug)]
pub struct Bool {
    repr: *mut libc::c_void,
}

impl Clone for Bool {
    fn clone(&self) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_copy(self.repr) };
        Self { repr }
    }
}

impl Bool {
    pub fn new(party: &mut Party, share: bool) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_new(party.repr, share) };
        Self { repr }
    }

    pub fn constant(party: &mut Party, value: bool) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_constant(party.repr, value) };
        Self { repr }
    }

    pub fn inv(&self) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_inv(self.repr) };
        Self { repr }
    }

    pub fn xor(&self, other: &Self) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_xor(self.repr, other.repr) };
        Self { repr }
    }

    pub fn mux(g: &Self, a: &Self, b: &Self) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_mux(g.repr, a.repr, b.repr) };
        Self { repr }
    }

    pub fn and(&self, other: &Self) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_and(self.repr, other.repr) };
        Self { repr }
    }

    pub fn get(&self) -> bool {
        unsafe { ffi::motion_gmw_bool_get(self.repr) }
    }
}

impl Drop for Bool {
    fn drop(&mut self) {
        unsafe { ffi::motion_gmw_bool_delete(self.repr) }
    }
}

#[derive(Debug)]
pub struct Nat {
    precision: usize,
    repr: *mut libc::c_void,
}

impl Clone for Nat {
    fn clone(&self) -> Self {
        let repr = unsafe { ffi::motion_gmw_nat_copy(self.repr) };
        Self {
            precision: self.precision,
            repr,
        }
    }
}

impl Nat {
    pub fn new(party: &mut Party, share: Vec<bool>) -> Self {
        let precision = share.len();
        let repr = unsafe { ffi::motion_gmw_nat_new(party.repr, share.as_ptr(), precision) };
        Self { precision, repr }
    }

    pub fn constant(party: &mut Party, value: Vec<bool>) -> Self {
        let precision = value.len();
        let repr = unsafe { ffi::motion_gmw_nat_constant(party.repr, value.as_ptr(), precision) };
        Self { precision, repr }
    }

    pub fn add(&self, other: &Self) -> Self {
        debug_assert_eq!(self.precision, other.precision);
        let precision = self.precision;
        let repr = unsafe { ffi::motion_gmw_nat_add(self.repr, other.repr) };
        Self { precision, repr }
    }

    pub fn sub(&self, other: &Self) -> Self {
        debug_assert_eq!(self.precision, other.precision);
        let precision = self.precision;
        let repr = unsafe { ffi::motion_gmw_nat_sub(self.repr, other.repr) };
        Self { precision, repr }
    }

    pub fn mul(&self, other: &Self) -> Self {
        debug_assert_eq!(self.precision, other.precision);
        let precision = self.precision;
        let repr = unsafe { ffi::motion_gmw_nat_mul(self.repr, other.repr) };
        Self { precision, repr }
    }

    pub fn mux(g: &Bool, a: &Self, b: &Self) -> Self {
        debug_assert_eq!(a.precision, b.precision);
        let precision = a.precision;
        let repr = unsafe { ffi::motion_gmw_nat_mux(g.repr, a.repr, b.repr) };
        Self { precision, repr }
    }

    pub fn eq(&self, other: &Self) -> Bool {
        debug_assert_eq!(self.precision, other.precision);
        let repr = unsafe { ffi::motion_gmw_nat_eq(self.repr, other.repr) };
        Bool { repr }
    }

    pub fn gt(&self, other: &Self) -> Bool {
        debug_assert_eq!(self.precision, other.precision);
        let repr = unsafe { ffi::motion_gmw_nat_gt(self.repr, other.repr) };
        Bool { repr }
    }

    pub fn get(&self) -> Vec<bool> {
        let precision = self.precision;
        let mut ret = Vec::with_capacity(precision);
        unsafe {
            ffi::motion_gmw_nat_get(self.repr, ret.as_mut_ptr(), precision);
            ret.set_len(precision);
        };
        ret
    }
}

impl Drop for Nat {
    fn drop(&mut self) {
        unsafe { ffi::motion_gmw_nat_delete(self.repr) }
    }
}

mod ffi {
    use super::*;

    extern "C" {
        pub fn motion_party_new(
            my_id: usize,
            hosts: *const *const libc::c_char,
            ports: *const u16,
            len: usize,
        ) -> *mut libc::c_void;

        pub fn motion_party_run(party: *mut libc::c_void);

        pub fn motion_party_delete(party: *mut libc::c_void);

        pub fn motion_gmw_bool_new(party: *mut libc::c_void, share: bool) -> *mut libc::c_void;

        pub fn motion_gmw_bool_constant(party: *mut libc::c_void, value: bool)
            -> *mut libc::c_void;

        pub fn motion_gmw_bool_copy(share: *mut libc::c_void) -> *mut libc::c_void;

        pub fn motion_gmw_bool_inv(share: *mut libc::c_void) -> *mut libc::c_void;

        pub fn motion_gmw_bool_xor(a: *mut libc::c_void, b: *mut libc::c_void)
            -> *mut libc::c_void;

        pub fn motion_gmw_bool_mux(
            g: *mut libc::c_void,
            a: *mut libc::c_void,
            b: *mut libc::c_void,
        ) -> *mut libc::c_void;

        pub fn motion_gmw_bool_and(a: *mut libc::c_void, b: *mut libc::c_void)
            -> *mut libc::c_void;

        pub fn motion_gmw_bool_get(share: *mut libc::c_void) -> bool;

        pub fn motion_gmw_bool_delete(share: *mut libc::c_void);

        pub fn motion_gmw_nat_new(
            party: *mut libc::c_void,
            share: *const bool,
            share_len: usize,
        ) -> *mut libc::c_void;

        pub fn motion_gmw_nat_constant(
            party: *mut libc::c_void,
            value: *const bool,
            value_len: usize,
        ) -> *mut libc::c_void;

        pub fn motion_gmw_nat_copy(share: *mut libc::c_void) -> *mut libc::c_void;

        pub fn motion_gmw_nat_add(a: *mut libc::c_void, b: *mut libc::c_void) -> *mut libc::c_void;

        pub fn motion_gmw_nat_sub(a: *mut libc::c_void, b: *mut libc::c_void) -> *mut libc::c_void;

        pub fn motion_gmw_nat_mul(a: *mut libc::c_void, b: *mut libc::c_void) -> *mut libc::c_void;

        pub fn motion_gmw_nat_eq(a: *mut libc::c_void, b: *mut libc::c_void) -> *mut libc::c_void;

        pub fn motion_gmw_nat_gt(a: *mut libc::c_void, b: *mut libc::c_void) -> *mut libc::c_void;

        pub fn motion_gmw_nat_mux(
            g: *mut libc::c_void,
            a: *mut libc::c_void,
            b: *mut libc::c_void,
        ) -> *mut libc::c_void;

        pub fn motion_gmw_nat_get(share: *mut libc::c_void, buf: *mut bool, buf_len: usize);

        pub fn motion_gmw_nat_delete(share: *mut libc::c_void);
    }
}
