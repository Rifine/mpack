use crate::traits::MsgPack;

use crate::format::MsgPackFormat;
use MsgPackFormat::*;

impl MsgPack for bool {
    fn pack_format(&self) -> MsgPackFormat {
        match self {
            true => True,
            false => False
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        buff.write_all(&[self.pack_format() as u8])
    }
}

impl<T: MsgPack> MsgPack for Option<T> {
    fn pack_format(&self) -> MsgPackFormat {
        match self {
            Some(v) => v.pack_format(),
            None => Nil
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Some(v) => v.pack_to(buff),
            None => buff.write_all(&[Nil as u8])
        }
    }
}

impl MsgPack for u8 {
    #[inline]
    fn pack_format(&self) -> MsgPackFormat {
        match self {
            0x00..0x80 => PositiveFixInt,
            _ => U8
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            0x00..0x80 => buff.write_all(&[PositiveFixInt as u8 | self]),
            _ => buff.write_all(&[U8 as u8, *self])
        }
    }
}

impl MsgPack for u16 {
    #[inline]
    fn pack_format(&self) -> MsgPackFormat {
        match self {
            0x00..0x80 => PositiveFixInt,
            0x80..0xff => U8,
            _ => U16,
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self.pack_format() {
            PositiveFixInt => buff.write_all(&[PositiveFixInt as u8 | *self as u8]),
            fmt => {
                buff.write_all(&[fmt as u8])?;
                buff.write_all(&self.to_be_bytes())
            }
        }
    }
}

impl MsgPack for u32 {
    #[inline]
    fn pack_format(&self) -> MsgPackFormat {
        match self {
            0x00..0x80 => PositiveFixInt,
            0x80..0xff => U8,
            0xff..0xffff => U16,
            _ => U32,
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self.pack_format() {
            PositiveFixInt => buff.write_all(&[PositiveFixInt as u8 | *self as u8]),
            fmt => {
                buff.write_all(&[fmt as u8])?;
                buff.write_all(&self.to_be_bytes())
            }
        }
    }
}

impl MsgPack for u64 {
    #[inline]
    fn pack_format(&self) -> MsgPackFormat {
        match self {
            0x00..0x80 => PositiveFixInt,
            0x80..0xff => U8,
            0xff..0xffff => U16,
            0xffff..0xffffffff => U32,
            _ => U64,
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self.pack_format() {
            PositiveFixInt => buff.write_all(&[PositiveFixInt as u8 | *self as u8]),
            fmt => {
                buff.write_all(&[fmt as u8])?;
                buff.write_all(&self.to_be_bytes())
            }
        }
    }
}

impl MsgPack for i8 {
    fn pack_format(&self) -> MsgPackFormat {
        match self {
            0..=i8::MAX => PositiveFixInt,
            -32..=-1 => NegativeFixInt,
            _ => I8,
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self.pack_format() {
            PositiveFixInt => buff.write_all(&[PositiveFixInt as u8 | *self as u8]),
            NegativeFixInt => buff.write_all(&[NegativeFixInt as u8 | (self + 32) as u8]),
            fmt => {
                buff.write_all(&[fmt as u8])?;
                buff.write_all(&self.to_be_bytes())
            }
        }
    }
}

impl MsgPack for i16 {
    fn pack_format(&self) -> MsgPackFormat {
        match self {
            0..=0x7f => PositiveFixInt,
            -32..=-1 => NegativeFixInt,
            _ => I16,
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self.pack_format() {
            PositiveFixInt => buff.write_all(&[PositiveFixInt as u8 | *self as u8]),
            NegativeFixInt => buff.write_all(&[NegativeFixInt as u8 | (self + 32) as u8]),
            fmt => {
                buff.write_all(&[fmt as u8])?;
                buff.write_all(&self.to_be_bytes())
            }
        }
    }
}

impl MsgPack for i32 {
    fn pack_format(&self) -> MsgPackFormat {
        const I16_MIN: i32 = i16::MIN as i32;
        const I16_MAX: i32 = i16::MAX as i32;
        match self {
            0..=0x7f => PositiveFixInt,
            -32..=-1 => NegativeFixInt,
            I16_MIN..I16_MAX => I16,
            _ => I32,
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self.pack_format() {
            PositiveFixInt => buff.write_all(&[PositiveFixInt as u8 | *self as u8]),
            NegativeFixInt => buff.write_all(&[NegativeFixInt as u8 | (self + 32) as u8]),
            fmt => {
                buff.write_all(&[fmt as u8])?;
                buff.write_all(&self.to_be_bytes())
            }
        }
    }
}

impl MsgPack for i64 {
    fn pack_format(&self) -> MsgPackFormat {
        const I16_MIN: i64 = i16::MIN as i64;
        const I16_MAX: i64 = i16::MAX as i64;
        const I32_MIN: i64 = i32::MIN as i64;
        const I32_MAX: i64 = i32::MAX as i64;
        match self {
            0..=0x7f => PositiveFixInt,
            -32..=-1 => NegativeFixInt,
            I16_MIN..I16_MAX => I16,
            I32_MIN..I32_MAX => I32,
            _ => I64,
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        match self.pack_format() {
            PositiveFixInt => buff.write_all(&[PositiveFixInt as u8 | *self as u8]),
            NegativeFixInt => buff.write_all(&[NegativeFixInt as u8 | (self + 32) as u8]),
            fmt => {
                buff.write_all(&[fmt as u8])?;
                buff.write_all(&self.to_be_bytes())
            }
        }
    }
}

impl MsgPack for f32 {
    fn pack_format(&self) -> MsgPackFormat {
        F32
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        buff.write_all(&[F32 as u8])?;
        buff.write_all(&self.to_be_bytes())
    }
}

impl MsgPack for f64 {
    fn pack_format(&self) -> MsgPackFormat {
        F64
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        buff.write_all(&[F64 as u8])?;
        buff.write_all(&self.to_be_bytes())
    }
}

impl MsgPack for String {
    fn pack_format(&self) -> MsgPackFormat {
        match self.len() {
            0..32 => FixStr,
            32..0x100 => Str8,
            0x100..0x10000 => Str16,
            _ => Str32,
        }
    }

    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()> {
        let len = self.len();
        match len {
            0..32 => {
                buff.write_all(&[FixStr as u8 | len as u8])?;
                buff.write_all(self.as_bytes())
            },
            32..0x100 => {
                buff.write_all(&[Str8 as u8])?;
                buff.write_all(&(len as u8).to_be_bytes())?;
                buff.write_all(self.as_bytes())
            },
            0x100..0x10000 => {
                buff.write_all(&[Str16 as u8])?;
                buff.write_all(&(len as u16).to_be_bytes())?;
                buff.write_all(self.as_bytes())
            },
            _ => {
                buff.write_all(&[Str32 as u8])?;
                buff.write_all(&(len as u32).to_be_bytes())?;
                buff.write_all(self.as_bytes())
            },
        }
    }
}