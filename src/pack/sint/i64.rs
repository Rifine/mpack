use super::{*, super::*};

impl Pack for i64 {
    #[inline]
    fn pack_format(&self) -> PackFormat {
        I64
    }

    #[inline]
    fn pack(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&[self.pack_format() as u8])?;
        w.write_all(&self.to_be_bytes())
    }
}

impl ConvertThenPack<bool> for i64 {
    #[inline]
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = self != &0;
        v.pack(w)
    }
}