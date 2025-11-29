use super::{*, super::*};

impl Pack for u64 {
    #[inline]
    fn pack_format(&self) -> PackFormat {
        U32
    }

    fn pack(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&[self.pack_format() as u8])?;
        w.write_all(&self.to_be_bytes())
    }
}