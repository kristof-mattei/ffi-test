#[derive(Debug)]
#[repr(C)]
pub struct Something {
    pub first: u8,
    pub second: u64,
}

#[link(name = "lib")]
extern "C" {
    pub fn build_something() -> Something;
}
