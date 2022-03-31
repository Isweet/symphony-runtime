pub mod eval;

use crate::bitvec::*;

pub trait Share: From<BV> + Into<BV> {}

impl Share for bool {}
impl Share for BV {}
impl Share for u32 {}
