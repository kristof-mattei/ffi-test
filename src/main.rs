mod ffi;

use std::slice::from_raw_parts;

use ffi::{build_something, Something};

unsafe fn serialize<T: Sized>(p: &T) -> &[u8] {
    from_raw_parts(std::ptr::from_ref::<T>(p).cast::<u8>(), size_of::<T>())
}

fn main() {
    from_ffi();
    from_rust();
}

fn from_ffi() {
    let s = unsafe { build_something() };

    dbg!(&s);

    let bytes: &[u8] = unsafe { serialize(&s) };

    println!("{:?}", bytes);
}

fn from_rust() {
    let s = Something {
        first: 1,
        second: u64::MAX,
    };

    dbg!(&s);

    let bytes: &[u8] = unsafe { serialize(&s) };

    println!("{:?}", bytes);
}
