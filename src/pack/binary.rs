use super::*;

impl Pack for Vec<u8> {
    fn pack_format(&self) -> PackFormat {
        match self.len() {
            0..U8_LIMIT => Bin8,
            U8_LIMIT..U16_LIMIT => Bin16,
            U16_LIMIT..U32_LIMIT => Bin32,
            _ => panic!("mpack: value too long")
        }
    }

    fn pack(&self, w: &mut impl Write) -> Result<()> {
        let len = self.len();
        match len {
            0..U8_LIMIT => {
                w.write_all(&[Bin8 as u8, len as u8])?;
                w.write_all(&self)
            },
            U8_LIMIT..U16_LIMIT => {
                w.write_all(&[Bin16 as u8])?;
                w.write_all(&(len as u16).to_be_bytes())?;
                w.write_all(&self)
            },
            U16_LIMIT..U32_LIMIT => {
                w.write_all(&[Bin32 as u8])?;
                w.write_all(&(len as u32).to_be_bytes())?;
                w.write_all(&self)
            },
            _ => panic!("mpack: value too long")
        }
    }
}

impl Pack for [u8] {
    fn pack_format(&self) -> PackFormat {
        match self.len() {
            0..U8_LIMIT => Bin8,
            U8_LIMIT..U16_LIMIT => Bin16,
            U16_LIMIT..U32_LIMIT => Bin32,
            _ => panic!("mpack: value too long")
        }
    }

    fn pack(&self, w: &mut impl Write) -> Result<()> {
        let len = self.len();
        match len {
            0..U8_LIMIT => {
                w.write_all(&[Bin8 as u8, len as u8])?;
                w.write_all(&self)
            },
            U8_LIMIT..U16_LIMIT => {
                w.write_all(&[Bin16 as u8])?;
                w.write_all(&(len as u16).to_be_bytes())?;
                w.write_all(&self)
            },
            U16_LIMIT..U32_LIMIT => {
                w.write_all(&[Bin32 as u8])?;
                w.write_all(&(len as u32).to_be_bytes())?;
                w.write_all(&self)
            },
            _ => panic!("mpack: value too long")
        }
    }
}