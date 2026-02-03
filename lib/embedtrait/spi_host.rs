use core::result::Result;

pub enum Data<'r, 'w> {
    Dummy(u8),
    Write(&'w [u8]),
    Read(&'w mut [u8]),
    Both { w: &'w [u8], r: &'r mut [u8] },
}

#[derive(Clone, Copy, Debug)]
pub enum Width {
    Single,
    Dual,
    Quad,
}

pub struct Operation<'r, 'w> {
    pub data: Data<'r, 'w>,
    pub width: Width,
}

pub trait SpiBus {
    type Error;
    fn transfer(&mut self, operations: &mut [Operation]) -> Result<(), Self::Error>;
}
