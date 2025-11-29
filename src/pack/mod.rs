use crate::{format::PackFormat};
use std::io::{Result, Write};
use PackFormat::*;

mod bool;
mod uint;
mod str;

pub trait ConvertThenPack<T: Pack> {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()>;
}

pub trait Pack {
    fn pack_format(&self) -> PackFormat;
    fn pack(&self, w: &mut impl Write) -> Result<()>;
    fn pack_as<T: Pack>(&self, w: &mut impl Write) -> Result<()>
    where Self: ConvertThenPack<T> {
        <Self as ConvertThenPack<T>>::convert_then_pack(self, w)
    }
}