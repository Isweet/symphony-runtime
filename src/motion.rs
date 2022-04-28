use std::cell::RefCell;
use std::net::TcpStream;
use std::os::unix::io::RawFd;
use std::os::unix::prelude::AsRawFd;
use std::rc::Rc;

pub struct Backend {
    my_id: usize,
    hosts: Vec<String>,
    ports: Vec<u16>,
    delayed: Vec<CachedBoolRef>,
    party: Party,
}

impl Backend {
    pub fn new(my_id: usize, hosts: Vec<String>, ports: Vec<u16>) -> Self {
        let party = Party::new(my_id, &hosts, &ports);
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
                CachedBool::Expr(e) => e.reify(),
                _ => unreachable!(),
            };
            *r = CachedBool::Value(share);
        }

        self.party = Party::new(self.my_id, &self.hosts, &self.ports);
    }
}

enum CachedBool {
    Value(bool),
    Constant(bool),
    Expr(BoolExpr),
}

impl CachedBool {
    fn expr(&self, backend: &mut Backend) -> BoolExpr {
        match self {
            CachedBool::Value(share) => BoolExpr::new(&mut backend.party, *share),
            CachedBool::Constant(value) => BoolExpr::constant(&mut backend.party, *value),
            CachedBool::Expr(e) => e.clone(),
        }
    }

    fn value(&self, backend: &mut Backend) -> Option<bool> {
        match self {
            CachedBool::Value(share) => Some(*share),
            CachedBool::Constant(value) => {
                let share = if backend.my_id == 0 { *value } else { false };
                Some(share)
            }
            CachedBool::Expr(_) => None,
        }
    }
}

type CachedBoolRef = Rc<RefCell<CachedBool>>;

pub struct Bool {
    repr: CachedBoolRef,
}

impl Bool {
    pub fn new(_backend: &mut Backend, share: bool) -> Self {
        let repr = Rc::new(RefCell::new(CachedBool::Value(share)));
        Self { repr }
    }

    pub fn constant(_backend: &mut Backend, value: bool) -> Self {
        let repr = Rc::new(RefCell::new(CachedBool::Constant(value)));
        Self { repr }
    }

    pub fn and(backend: &mut Backend, a: &mut Self, b: &mut Self) -> Self {
        let mut expr_a = (&*a.repr.borrow()).expr(backend);
        let mut expr_b = (&*b.repr.borrow()).expr(backend);
        let expr = expr_a.and(&mut expr_b);
        let repr = Rc::new(RefCell::new(CachedBool::Expr(expr)));
        backend.delayed.push(repr.clone());
        Self { repr }
    }

    pub fn reify(backend: &mut Backend, value: &mut Self) -> bool {
        let cached = (&*value.repr.borrow()).value(backend);
        match cached {
            None => {
                backend.run();
                (&*value.repr.borrow()).value(backend).unwrap()
            }
            Some(share) => share,
        }
    }
}

// MOTION Party

struct Party {
    repr: *mut libc::c_void,
}

impl Party {
    fn new(my_id: usize, hosts: &[String], ports: &[u16]) -> Self {
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

    fn run(&mut self) {
        unsafe { ffi::motion_party_run(self.repr) }
    }
}

impl Drop for Party {
    fn drop(&mut self) {
        unsafe { ffi::motion_party_delete(self.repr) }
    }
}

// MOTION Boolean Expression

#[derive(Clone)]
struct BoolExpr {
    repr: *mut libc::c_void,
}

impl BoolExpr {
    fn new(party: &mut Party, share: bool) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_new(party.repr, share) };
        Self { repr }
    }

    fn constant(party: &mut Party, value: bool) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_constant(party.repr, value) };
        Self { repr }
    }

    fn and(&mut self, other: &mut Self) -> Self {
        let repr = unsafe { ffi::motion_gmw_bool_and(self.repr, other.repr) };
        Self { repr }
    }

    fn reify(&mut self) -> bool {
        unsafe { ffi::motion_gmw_bool_reify(self.repr) }
    }
}

impl Drop for BoolExpr {
    fn drop(&mut self) {
        unsafe { ffi::motion_gmw_bool_delete(self.repr) }
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

        pub fn motion_gmw_bool_and(a: *mut libc::c_void, b: *mut libc::c_void)
            -> *mut libc::c_void;

        pub fn motion_gmw_bool_reify(share: *mut libc::c_void) -> bool;

        pub fn motion_gmw_bool_delete(share: *mut libc::c_void);
    }
}
