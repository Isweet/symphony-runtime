use rand::{CryptoRng, Rng};
use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;

pub trait Sec<T> {
    type Item;
}

pub trait SecIO<Prg: Rng + CryptoRng, T>: Sec<T> {
    fn send_input(prg: &mut Prg, parties: &mut Vec<Box<dyn Write>>, input: &T);
    fn recv_input(&mut self, client: &mut Box<dyn Read>) -> Self::Item;

    fn send_input_secure(prg: &mut Prg, parties: &mut Vec<Box<dyn Write>>, input: &Self::Item);
    fn recv_input_secure(&mut self, clients: &mut Vec<Box<dyn Read>>) -> Self::Item;

    fn send_output(&mut self, client: &mut Box<dyn Write>, output: &Self::Item);
    fn recv_output(parties: &mut Vec<Box<dyn Read>>) -> T;
}

pub trait SecLit<T>: Sec<T> {
    fn sec_lit(&mut self, constant: &T) -> Self::Item;
}

pub trait SecAddId<T>: Sec<T> {
    fn sec_add_id(&mut self) -> Self::Item;
}

pub trait SecAddInv<T>: Sec<T> {
    fn sec_add_inv(&mut self, a: &Self::Item) -> Self::Item;
}

pub trait SecAdd<T>: Sec<T> {
    fn sec_add(&mut self, a: &Self::Item, b: &Self::Item) -> Self::Item;
}

pub trait SecSub<T>: Sec<T> {
    fn sec_sub(&mut self, a: &Self::Item, b: &Self::Item) -> Self::Item;
}

pub trait SecMulId<T>: Sec<T> {
    fn sec_mul_id(&mut self) -> Self::Item;
}

pub trait SecMulInv<T>: Sec<T> {
    fn sec_mul_inv(&mut self, a: &Self::Item) -> Self::Item;
}

pub trait SecMul<T>: Sec<T> {
    fn sec_mul(&mut self, a: &Self::Item, b: &Self::Item) -> Self::Item;
}

pub trait SecDiv<T>: Sec<T> {
    fn sec_div(&mut self, a: &Self::Item, b: &Self::Item) -> Self::Item;
}

pub trait SecMod<T>: Sec<T> {
    fn sec_mod(&mut self, a: &Self::Item, b: &Self::Item) -> Self::Item;
}

pub trait SecEq<T>: Sec<bool> + Sec<T> {
    fn sec_eq(
        &mut self,
        a: &<Self as Sec<T>>::Item,
        b: &<Self as Sec<T>>::Item,
    ) -> <Self as Sec<bool>>::Item;
}

pub trait SecNeq<T>: Sec<bool> + Sec<T> {
    fn sec_neq(
        &mut self,
        a: &<Self as Sec<T>>::Item,
        b: &<Self as Sec<T>>::Item,
    ) -> <Self as Sec<bool>>::Item;
}

pub trait SecLt<T>: Sec<bool> + Sec<T> {
    fn sec_lt(
        &mut self,
        a: &<Self as Sec<T>>::Item,
        b: &<Self as Sec<T>>::Item,
    ) -> <Self as Sec<bool>>::Item;
}

pub trait SecLte<T>: Sec<bool> + Sec<T> {
    fn sec_lte(
        &mut self,
        a: &<Self as Sec<T>>::Item,
        b: &<Self as Sec<T>>::Item,
    ) -> <Self as Sec<bool>>::Item;
}

pub trait SecGt<T>: Sec<bool> + Sec<T> {
    fn sec_gt(
        &mut self,
        a: &<Self as Sec<T>>::Item,
        b: &<Self as Sec<T>>::Item,
    ) -> <Self as Sec<bool>>::Item;
}

pub trait SecGte<T>: Sec<bool> + Sec<T> {
    fn sec_gte(
        &mut self,
        a: &<Self as Sec<T>>::Item,
        b: &<Self as Sec<T>>::Item,
    ) -> <Self as Sec<bool>>::Item;
}

pub trait SecCond<T>: Sec<bool> + Sec<T> {
    fn sec_cond(
        &mut self,
        a: &<Self as Sec<bool>>::Item,
        b: &<Self as Sec<T>>::Item,
        c: &<Self as Sec<T>>::Item,
    ) -> <Self as Sec<T>>::Item;
}
