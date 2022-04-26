use std::io::Read;
use std::io::Write;

pub fn read_bool<R: Read>(r: &mut R) -> std::io::Result<bool> {
    let mut buf = [0u8];
    r.read_exact(&mut buf).map(|()| buf[0] != 0)
}

pub fn write_bool<W: Write>(w: &mut W, b: bool) -> std::io::Result<()> {
    w.write_all(&[b as u8])
}

pub mod ffi {
    pub unsafe fn c_to_vec<T: Clone>(data: *const T, len: usize) -> Vec<T> {
        let mut ret = Vec::with_capacity(len);

        for i in 0..len {
            let element_ref = &*data.add(i);
            ret.push(element_ref.clone())
        }

        ret
    }
}
