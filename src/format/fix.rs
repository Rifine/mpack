use crate::format::sealed::MessagePackFormat;

/// Format marker type.
/// 
/// # Example
/// ```
/// use mpack::{Pack, Binary, format::Bin8, MessagePackFormatInfo};
/// let mut buff = Vec::new();
/// let bytes = b"Hello world";
/// bytes.pack::<Binary>(&mut buff).unwrap();
/// assert_eq!([Bin8::MARKER, 11], buff[0..2]);
/// assert_eq!(*bytes, buff[2..]);
/// ```
pub enum Binary {}

pub enum SignedInt {}
pub enum UnsignedInt {}

impl MessagePackFormat for Binary {}
impl MessagePackFormat for String {}
