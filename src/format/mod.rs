pub mod fix;
mod marker;
mod mask;

pub use fix::*;

pub(crate) mod sealed {
    pub trait MessagePackFormat {}
}

pub trait MessagePackFormatInfo: sealed::MessagePackFormat {
    const MARKER: u8;
    const MASK: Option<u8> = None;
}

pub trait ExtendFormat {
    const TYPE: u8;
}
impl<T: ExtendFormat> sealed::MessagePackFormat for T {}

pub enum PosFixInt {}
pub enum FixMap {}
pub enum FixArray {}
pub enum FixStr {}
pub enum Nil {}
pub enum Bin8 {}
pub enum Bin16 {}
pub enum Bin32 {}
pub enum Ext8 {}
pub enum Ext16 {}
pub enum Ext32 {}
pub enum FixExt1 {}
pub enum FixExt2 {}
pub enum FixExt4 {}
pub enum FixExt8 {}
pub enum FixExt16 {}
pub enum Str8 {}
pub enum Str16 {}
pub enum Str32 {}
pub enum Array16 {}
pub enum Aarray32 {}
pub enum Map16 {}
pub enum Map32 {}
pub enum NegFixInt {}

impl sealed::MessagePackFormat for PosFixInt {}
impl MessagePackFormatInfo for PosFixInt {
    const MARKER: u8 = marker::POS_FIX_INT;
    const MASK: Option<u8> = mask::POS_FIX_INT;
}
impl sealed::MessagePackFormat for FixMap {}
impl MessagePackFormatInfo for FixMap {
    const MARKER: u8 = marker::FIX_MAP;
    const MASK: Option<u8> = mask::FIX_MAP;
}
impl sealed::MessagePackFormat for FixArray {}
impl MessagePackFormatInfo for FixArray {
    const MARKER: u8 = marker::FIX_ARRAY;
    const MASK: Option<u8> = mask::FIX_ARRAY;
}
impl sealed::MessagePackFormat for FixStr {}
impl MessagePackFormatInfo for FixStr {
    const MARKER: u8 = marker::FIX_STR;
    const MASK: Option<u8> = mask::FIX_STR;
}
impl sealed::MessagePackFormat for Nil {}
impl MessagePackFormatInfo for Nil {
    const MARKER: u8 = marker::NIL;
}
impl sealed::MessagePackFormat for bool {}
impl MessagePackFormatInfo for bool {
    const MARKER: u8 = marker::FALSE & marker::TRUE;
    const MASK: Option<u8> = mask::BOOL;
}
impl sealed::MessagePackFormat for Bin8 {}
impl MessagePackFormatInfo for Bin8 {
    const MARKER: u8 = marker::BIN8;
}
impl sealed::MessagePackFormat for Bin16 {}
impl MessagePackFormatInfo for Bin16 {
    const MARKER: u8 = marker::BIN16;
}
impl sealed::MessagePackFormat for Bin32 {}
impl MessagePackFormatInfo for Bin32 {
    const MARKER: u8 = marker::BIN32;
}
impl sealed::MessagePackFormat for Ext8 {}
impl MessagePackFormatInfo for Ext8 {
    const MARKER: u8 = marker::EXT8;
}
impl sealed::MessagePackFormat for Ext16 {}
impl MessagePackFormatInfo for Ext16 {
    const MARKER: u8 = marker::EXT16;
}
impl sealed::MessagePackFormat for Ext32 {}
impl MessagePackFormatInfo for Ext32 {
    const MARKER: u8 = marker::EXT32;
}
impl sealed::MessagePackFormat for f32 {}
impl MessagePackFormatInfo for f32 {
    const MARKER: u8 = marker::F32;
}
impl sealed::MessagePackFormat for f64 {}
impl MessagePackFormatInfo for f64 {
    const MARKER: u8 = marker::F64;
}
impl sealed::MessagePackFormat for u8 {}
impl MessagePackFormatInfo for u8 {
    const MARKER: u8 = marker::U8;
}
impl sealed::MessagePackFormat for u16 {}
impl MessagePackFormatInfo for u16 {
    const MARKER: u8 = marker::U16;
}
impl sealed::MessagePackFormat for u32 {}
impl MessagePackFormatInfo for u32 {
    const MARKER: u8 = marker::U32;
}
impl sealed::MessagePackFormat for u64 {}
impl MessagePackFormatInfo for u64 {
    const MARKER: u8 = marker::U64;
}
impl sealed::MessagePackFormat for i8 {}
impl MessagePackFormatInfo for i8 {
    const MARKER: u8 = marker::I8;
}
impl sealed::MessagePackFormat for i16 {}
impl MessagePackFormatInfo for i16 {
    const MARKER: u8 = marker::I16;
}
impl sealed::MessagePackFormat for i32 {}
impl MessagePackFormatInfo for i32 {
    const MARKER: u8 = marker::I32;
}
impl sealed::MessagePackFormat for i64 {}
impl MessagePackFormatInfo for i64 {
    const MARKER: u8 = marker::I64;
}
impl sealed::MessagePackFormat for FixExt1 {}
impl MessagePackFormatInfo for FixExt1 {
    const MARKER: u8 = marker::FIX_EXT1;
}
impl sealed::MessagePackFormat for FixExt2 {}
impl MessagePackFormatInfo for FixExt2 {
    const MARKER: u8 = marker::FIX_EXT2;
}
impl sealed::MessagePackFormat for FixExt4 {}
impl MessagePackFormatInfo for FixExt4 {
    const MARKER: u8 = marker::FIX_EXT4;
}
impl sealed::MessagePackFormat for FixExt8 {}
impl MessagePackFormatInfo for FixExt8 {
    const MARKER: u8 = marker::FIX_EXT8;
}
impl sealed::MessagePackFormat for FixExt16 {}
impl MessagePackFormatInfo for FixExt16 {
    const MARKER: u8 = marker::FIX_EXT16;
}
impl sealed::MessagePackFormat for Str8 {}
impl MessagePackFormatInfo for Str8 {
    const MARKER: u8 = marker::STR8;
}
impl sealed::MessagePackFormat for Str16 {}
impl MessagePackFormatInfo for Str16 {
    const MARKER: u8 = marker::STR16;
}
impl sealed::MessagePackFormat for Str32 {}
impl MessagePackFormatInfo for Str32 {
    const MARKER: u8 = marker::STR32;
}
impl sealed::MessagePackFormat for Array16 {}
impl MessagePackFormatInfo for Array16 {
    const MARKER: u8 = marker::ARRAY16;
}
impl sealed::MessagePackFormat for Aarray32 {}
impl MessagePackFormatInfo for Aarray32 {
    const MARKER: u8 = marker::ARRAY32;
}
impl sealed::MessagePackFormat for Map16 {}
impl MessagePackFormatInfo for Map16 {
    const MARKER: u8 = marker::MAP16;
}
impl sealed::MessagePackFormat for Map32 {}
impl MessagePackFormatInfo for Map32 {
    const MARKER: u8 = marker::MAP32;
}
impl sealed::MessagePackFormat for NegFixInt {}
impl MessagePackFormatInfo for NegFixInt {
    const MARKER: u8 = marker::NEG_FIX_INT;
    const MASK: Option<u8> = mask::NEG_FIX_INT;
}
