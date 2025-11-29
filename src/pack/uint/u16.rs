use super::{*, super::*};

impl Pack for u16 {
    #[inline]
    fn pack_format(&self) -> PackFormat {
        U16
    }

    fn pack(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&[self.pack_format() as u8])?;
        w.write_all(&self.to_be_bytes())
    }
}

impl ConvertThenPack<u32> for u16 {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u32;
        v.pack(w)
    }
}

impl ConvertThenPack<u64> for u16 {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u64;
        v.pack(w)
    }
}