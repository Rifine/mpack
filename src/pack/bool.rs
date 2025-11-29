use super::*;

impl Pack for bool {
    fn pack_format(&self) -> PackFormat {
        match self {
            true => True,
            _ => False
        }
    }

    fn pack(&self, w: &mut impl Write) -> Result<()> {
        match self {
            true => w.write_all(&[True as u8]),
            _ => w.write_all(&[False as u8])
        }
    }
}

impl ConvertThenPack<u8> for bool {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u8;
        v.pack(w)
    }
}

impl ConvertThenPack<u16> for bool {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u16;
        v.pack(w)
    }
}

impl ConvertThenPack<u32> for bool {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u32;
        v.pack(w)
    }
}

impl ConvertThenPack<u64> for bool {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        let v = *self as u64;
        v.pack(w)
    }
}

impl ConvertThenPack<String> for bool {
    fn convert_then_pack(&self, w: &mut impl Write) -> Result<()> {
        match self {
            true => "true".pack(w),
            _ => "false".pack(w)
        }
    }
}