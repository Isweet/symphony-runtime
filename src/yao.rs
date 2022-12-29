mod emp {
    pub struct NetIo {
        repr: *mut libc::c_void,
    }

    impl NetIo {
        pub fn new_server(port: u16) -> Self {
            let repr = unsafe { ffi::emp_netio_new(std::ptr::null(), port) };
            Self { repr }
        }

        pub fn new_client(host: String, port: u16) -> Self {
            let c_host = std::ffi::CString::new(host).expect("TODO").as_ptr();
            let repr = unsafe { ffi::emp_netio_new(c_host, port) };
            Self { repr }
        }
    }

    impl Drop for NetIo {
        fn drop(&mut self) {
            unsafe { ffi::emp_netio_delete(self.repr) }
        }
    }

    pub struct Context {
        net: NetIo,
        circuitExe: *mut libc::c_void,
        protocolExe: *mut libc::c_void,
    }

    impl Context {
        pub fn new(party: i8, net: NetIo) -> Self {
            unsafe { ffi::emp_setup_semi_honest(net.repr, party) };
            let circuitExe = unsafe { ffi::emp_get_circuit_exe() };
            let protocolExe = unsafe { ffi::emp_get_protocol_exe() };
            Self {
                net,
                circuitExe,
                protocolExe,
            }
        }

        fn set_context(&mut self) {
            unsafe { ffi::emp_set_exe(self.circuitExe, self.protocolExe) }
        }
    }

    impl Drop for Context {
        fn drop(&mut self) {
            self.set_context();
            unsafe { ffi::emp_finalize_semi_honest() }
        }
    }

    pub struct Bool {
        repr: *mut libc::c_void,
    }

    impl Bool {
        pub fn new(context: &mut Context, share: bool) -> Self {
            context.set_context();
            let repr = unsafe { ffi::emp_yao_bool_new(share) };
            Self { repr }
        }
    }

    mod ffi {
        extern "C" {
            pub fn emp_netio_new(host: *const libc::c_char, port: u16) -> *mut libc::c_void;
            pub fn emp_netio_delete(netio: *mut libc::c_void);

            pub fn emp_setup_semi_honest(net: *mut libc::c_void, party: i8);
            pub fn emp_get_circuit_exe() -> *mut libc::c_void;
            pub fn emp_get_protocol_exe() -> *mut libc::c_void;
            pub fn emp_set_exe(circuitExe: *mut libc::c_void, protocolExe: *mut libc::c_void);
            pub fn emp_finalize_semi_honest();

            pub fn emp_yao_bool_new(share: bool) -> *mut libc::c_void;
        }
    }
}
