use super::{*, super::*};

impl Pack for u32 {
    #[inline]
    fn pack_format(&self) -> PackFormat {
        U32
    }

    #[inline]
    fn pack(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&[self.pack_format() as u8])?;
        w.write_all(&self.to_be_bytes())
    }
}

impl ConvertThenPack<u64> for u32 {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u64;
        v.pack(w)
    }
}