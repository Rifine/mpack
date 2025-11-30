use super::*;

impl Pack for Array {
    #[inline]
    fn pack_format(&self) -> PackFormat {
        match self.len {
            0..16 => FixArray,
            16..U16_LIMIT => Array16,
            U16_LIMIT..U32_LIMIT => Array32,
            _ => panic!("mpack: value too long")
        }
    }

    #[inline]
    fn pack(&self, w: &mut impl Write) -> Result<()> {
        match self.len {
            0..16 => {
                w.write_all(&[FixArray as u8 | self.len as u8])?;
                for _ in 0..self.len {
                    w.write_all(&[Nil as u8])?;
                }
                Ok(())
            },
            16..U16_LIMIT => {
                w.write_all(&[Array16 as u8])?;
                w.write_all(&(self.len as u16).to_be_bytes())?;
                for _ in 0..self.len {
                    w.write_all(&[Nil as u8])?;
                }
                Ok(())
            },
            U16_LIMIT..U32_LIMIT => {
                w.write_all(&[Array32 as u8])?;
                w.write_all(&(self.len as u32).to_be_bytes())?;
                for _ in 0..self.len {
                    w.write_all(&[Nil as u8])?;
                }
                Ok(())
            },
            _ => panic!("mpack: value too long")
        }
    }
}

impl<T: Pack> ConvertThenPack<Array> for Vec<T> {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let arr = Array { len: self.len() };
        match arr.len {
            0..16 => {
                w.write_all(&[FixArray as u8 | arr.len as u8])?;
                for v in self {
                    v.pack(w)?;
                }
                Ok(())
            },
            16..U16_LIMIT => {
                w.write_all(&[Array16 as u8])?;
                w.write_all(&(arr.len as u16).to_be_bytes())?;
                for v in self {
                    v.pack(w)?;
                }
                Ok(())
            },
            U16_LIMIT..U32_LIMIT => {
                w.write_all(&[Array32 as u8])?;
                w.write_all(&(arr.len as u32).to_be_bytes())?;
                for v in self {
                    v.pack(w)?;
                }
                Ok(())
            },
            _ => panic!("mpack: value too long")
        }
    }
}

impl<T: Pack> ConvertThenPack<Array> for [T] {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let arr = Array { len: self.len() };
        match arr.len {
            0..16 => {
                w.write_all(&[FixArray as u8 | arr.len as u8])?;
                for v in self {
                    v.pack(w)?;
                }
                Ok(())
            },
            16..U16_LIMIT => {
                w.write_all(&[Array16 as u8])?;
                w.write_all(&(arr.len as u16).to_be_bytes())?;
                for v in self {
                    v.pack(w)?;
                }
                Ok(())
            },
            U16_LIMIT..U32_LIMIT => {
                w.write_all(&[Array32 as u8])?;
                w.write_all(&(arr.len as u32).to_be_bytes())?;
                for v in self {
                    v.pack(w)?;
                }
                Ok(())
            },
            _ => panic!("mpack: value too long")
        }
    }
}