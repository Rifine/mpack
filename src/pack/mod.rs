use crate::{format::PackFormat};
use std::io::{Result, Write};
use PackFormat::*;

mod bool;
mod uint;
mod sint;
mod str;
mod array;
mod binary;

const U8_LIMIT: usize = u8::MAX as usize + 1;
const U16_LIMIT: usize = u16::MAX as usize + 1;
const U32_LIMIT: usize = u32::MAX as usize + 1;

pub trait ConvertThenPack<T: Pack> {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()>;
}

pub trait PackAs {
    fn pack_as<T: Pack>(&self, w: &mut impl Write) -> Result<()>
    where Self: ConvertThenPack<T>;
}

pub trait Pack {
    fn pack_format(&self) -> PackFormat;
    fn pack(&self, w: &mut impl Write) -> Result<()>;
    fn pack_as<T: Pack>(&self, w: &mut impl Write) -> Result<()>
    where Self: ConvertThenPack<T> {
        <Self as ConvertThenPack<T>>::convert_then_pack(self, w)
    }
}

impl<T: Pack> ConvertThenPack<T> for T {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        self.pack(w)
    }
}

pub struct Array { pub len: usize }
pub struct Binary { pub len: usize }
pub struct Map { pub len: usize }