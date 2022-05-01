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
        let repr = unsafe {
            ffi::motion_party_new(my_id, c_hosts_ptrs.as_ptr(), ports.as_ptr(), hosts.len())
        };
        Self { repr }
    }

    pub fn run(&self) {
        unsafe { ffi::motion_party_run(self.repr) }
    }
}

impl Drop for Party {
    fn drop(&mut self) {
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

    pub fn xor(&self, other: &Self) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_xor(self.repr, other.repr) };
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

mod ffi {
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

        pub fn motion_gmw_bool_xor(a: *mut libc::c_void, b: *mut libc::c_void)
            -> *mut libc::c_void;

        pub fn motion_gmw_bool_and(a: *mut libc::c_void, b: *mut libc::c_void)
            -> *mut libc::c_void;

        pub fn motion_gmw_bool_get(share: *mut libc::c_void) -> bool;

        pub fn motion_gmw_bool_delete(share: *mut libc::c_void);
    }
}
