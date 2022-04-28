pub mod ffi {
    use super::*;
    use rand::Rng;
    use rand::SeedableRng;
    use scuttlebutt::AesRng;
    use scuttlebutt::Block;

    #[no_mangle]
    pub unsafe extern "C" fn prg_drop(this: *mut AesRng) {
        Box::from_raw(this);
    }

    #[no_mangle]
    pub extern "C" fn prg_new() -> *mut AesRng {
        Box::into_raw(Box::new(AesRng::new()))
    }

    #[no_mangle]
    pub extern "C" fn prg_from_seed(seed1: u64, seed2: u64) -> *mut AesRng {
        let mut seed = [0; 16];
        for (i, b) in seed1.to_ne_bytes().into_iter().enumerate() {
            seed[i] = b;
        }

        for (i, b) in seed2.to_ne_bytes().into_iter().enumerate() {
            seed[i + 8] = b;
        }

        Box::into_raw(Box::new(AesRng::from_seed(Block::from(seed))))
    }

    #[no_mangle]
    pub unsafe extern "C" fn prg_rand_bool(this: *mut AesRng) -> bool {
        (*this).gen()
    }
}
