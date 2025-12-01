use std::io::Write;

use crate::MPackResult as Result;
use crate::format::{sealed::*, *};

pub trait Pack {
    fn pack<F: MessagePackFormat>(&self, buff: &mut impl Write) -> Result<()>
    where
        Self: PackAs<F>;
}

impl<T> Pack for T {
    fn pack<F: MessagePackFormat>(&self, buff: &mut impl Write) -> Result<()>
    where
        Self: PackAs<F>,
    {
        self.pack_as(buff)
    }
}

pub trait PackAs<F: MessagePackFormat> {
    fn pack_as(&self, buff: &mut impl Write) -> Result<()>;
}

impl<T, F> PackAs<F> for Option<T>
where
    T: PackAs<F>,
    F: MessagePackFormat,
{
    fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
        if let Some(v) = self {
            v.pack_as(buff)
        } else {
            buff.write_all(&[Nil::MARKER])?;
            Ok(())
        }
    }
}

impl PackAs<Nil> for Nil {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
        buff.write_all(&[Nil::MARKER])?;
        Ok(())
    }
}

impl PackAs<bool> for bool {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
        buff.write_all(&[Self::MARKER | *self as u8])?;
        Ok(())
    }
}

impl PackAs<u8> for u8 {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
        buff.write_all(&[Self::MARKER, *self])?;
        Ok(())
    }
}

impl PackAs<u16> for u16 {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
        let bytes: [u8; 2] = self.to_be_bytes();
        let bytes: [u8; 3] = [Self::MARKER, bytes[0], bytes[1]];
        buff.write_all(&bytes)?;
        Ok(())
    }
}

impl PackAs<u32> for u32 {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
        let bytes: [u8; 4] = self.to_be_bytes();
        let bytes: [u8; 5] = [Self::MARKER, bytes[0], bytes[1], bytes[2], bytes[3]];
        buff.write_all(&bytes)?;
        Ok(())
    }
}

impl PackAs<u64> for u64 {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
        let bytes: [u8; 8] = self.to_be_bytes();
        let bytes: [u8; 9] = [Self::MARKER, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]];
        buff.write_all(&bytes)?;
        Ok(())
    }
}