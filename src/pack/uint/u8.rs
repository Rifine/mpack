use super::{*, super::*};

impl Pack for u8 {
    #[inline]
    fn pack_format(&self) -> PackFormat {
        U8
    }

    #[inline]
    fn pack(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&[self.pack_format() as u8, *self])
    }
}

impl ConvertThenPack<u16> for u8 {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u16;
        v.pack(w)
    }
}

impl ConvertThenPack<u32> for u8 {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u32;
        v.pack(w)
    }
}

impl ConvertThenPack<u64> for u8 {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u64;
        v.pack(w)
    }
}