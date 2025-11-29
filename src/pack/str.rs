use super::*;

const U8_LIMIT: usize = u8::MAX as usize + 1;
const U16_LIMIT: usize = u16::MAX as usize + 1;
const U32_LIMIT: usize = u32::MAX as usize + 1;

impl Pack for String {
    fn pack_format(&self) -> PackFormat {
        match self.len() {
            0..32 => FixStr,
            32..U8_LIMIT => Str8,
            U8_LIMIT..U16_LIMIT => Str16,
            U16_LIMIT..U32_LIMIT => Str32,
            _ => panic!("mpack: value too long!")
        }
    }

    fn pack(&self, w: &mut impl Write) -> Result<()> {
        let len = self.len();
        match len {
            0..32 => {
                w.write_all(&[FixStr as u8 | (len as u8)])?;
                w.write_all(&self.as_bytes())
            },
            32..U8_LIMIT => {
                w.write_all(&[Str8 as u8, len as u8])?;
                w.write_all(&self.as_bytes())
            },
            U8_LIMIT..U16_LIMIT => {
                w.write_all(&[Str16 as u8])?;
                w.write_all(&(len as u16).to_be_bytes())?;
                w.write_all(&self.as_bytes())
            },
            U16_LIMIT..U32_LIMIT => {
                w.write_all(&[Str32 as u8])?;
                w.write_all(&(len as u32).to_be_bytes())?;
                w.write_all(&self.as_bytes())
            },
            _ => panic!("mpack: value too long!")
        }
    }
}

impl Pack for str {
    fn pack_format(&self) -> PackFormat {
        match self.len() {
            0..32 => FixStr,
            32..U8_LIMIT => Str8,
            U8_LIMIT..U16_LIMIT => Str16,
            U16_LIMIT..U32_LIMIT => Str32,
            _ => panic!("mpack: value too long!")
        }
    }

    fn pack(&self, w: &mut impl Write) -> Result<()> {
        let len = self.len();
        match len {
            0..32 => {
                w.write_all(&[FixStr as u8 | (len as u8)])?;
                w.write_all(&self.as_bytes())
            },
            32..U8_LIMIT => {
                w.write_all(&[Str8 as u8, len as u8])?;
                w.write_all(&self.as_bytes())
            },
            U8_LIMIT..U16_LIMIT => {
                w.write_all(&[Str16 as u8])?;
                w.write_all(&(len as u16).to_be_bytes())?;
                w.write_all(&self.as_bytes())
            },
            U16_LIMIT..U32_LIMIT => {
                w.write_all(&[Str32 as u8])?;
                w.write_all(&(len as u32).to_be_bytes())?;
                w.write_all(&self.as_bytes())
            },
            _ => panic!("mpack: value too long!")
        }
    }
}