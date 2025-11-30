use super::{*, super::*};

impl Pack for i8 {
    #[inline]
    fn pack_format(&self) -> PackFormat {
        I8
    }

    #[inline]
    fn pack(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&[self.pack_format() as u8])?;
        w.write_all(&self.to_be_bytes())
    }
}

impl ConvertThenPack<i16> for i8 {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as i16;
        v.pack(w)
    }
}

impl ConvertThenPack<i32> for i8 {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as i32;
        v.pack(w)
    }
}

impl ConvertThenPack<i64> for i8 {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as i64;
        v.pack(w)
    }
}