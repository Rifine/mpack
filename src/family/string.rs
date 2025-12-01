use crate::{
    MPackError, MessagePackFormatInfo,
    format::{FixStr, Str8, Str16, Str32},
    pack::PackAs,
};

/// A family of types that can be represented as string data in MessagePack format.
///
/// Types implementing this trait provide access to their UTF-8 byte representation,
/// which can be serialized as MessagePack string format (fixstr, str8, str16, str32).
///
/// This trait allows uniform handling of different string-like types such as
/// [`String`], `&str`, and string slices for MessagePack serialization.
///
/// # Implements
/// Implements [`PackAs<String>`] for all types in the [`StringFamily`].
///
/// This provides automatic string serialization using the appropriate
/// MessagePack string format based on data length:
/// - fixstr for data up to 31 bytes
/// - str8 for data up to [`u8::MAX`] bytes
/// - str16 for data up to [`u16::MAX`] bytes
/// - str32 for data up to [`u32::MAX`] bytes
pub trait StringFamily {
    /// Returns a slice containing the UTF-8 bytes to be packed as MessagePack string data.
    ///
    /// # Returns
    /// A byte slice representing the UTF-8 encoded string to be serialized.
    ///
    /// # Panics
    /// Implementations must ensure the returned bytes are valid UTF-8.
    ///
    /// Invalid UTF-8 data will cause serialization errors.
    fn str_to_pack(&self) -> &str;
}

impl StringFamily for String {
    #[inline(always)]
    fn str_to_pack(&self) -> &str {
        self.as_str()
    }
}

impl StringFamily for str {
    #[inline(always)]
    fn str_to_pack(&self) -> &str {
        self
    }
}

impl StringFamily for &str {
    #[inline(always)]
    fn str_to_pack(&self) -> &str {
        self
    }
}

impl<'a> StringFamily for std::borrow::Cow<'a, str> {
    #[inline(always)]
    fn str_to_pack(&self) -> &str {
        self.as_ref()
    }
}

impl<'a> StringFamily for Box<str> {
    #[inline(always)]
    fn str_to_pack(&self) -> &str {
        self.as_ref()
    }
}

impl<S: StringFamily> PackAs<String> for S {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let bytes = self.str_to_pack().as_bytes();
        let len = bytes.len();

        let prefix: &[u8] = match len {
            0xffffff.. => return Err(MPackError::size_exceeded(len, u32::MAX as usize)),
            0xffff.. => {
                let prefix: [u8; 4] = (len as u32).to_be_bytes();
                &[Str32::MARKER, prefix[0], prefix[1], prefix[2], prefix[3]]
            }
            0xff.. => {
                let prefix: [u8; 2] = (len as u16).to_be_bytes();
                &[Str16::MARKER, prefix[0], prefix[1]]
            }
            0x20.. => &[Str8::MARKER, len as u8],
            _ => &[FixStr::MARKER | len as u8],
        };
        buff.write_all(prefix)?;
        buff.write_all(bytes)?;
        Ok(())
    }
}

impl<S: StringFamily> PackAs<FixStr> for S {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let str_data = self.str_to_pack();
        let bytes = str_data.as_bytes();
        let len = bytes.len();

        if len > 31 {
            return Err(MPackError::size_exceeded(len, 31));
        }

        buff.write_all(&[FixStr::MARKER | len as u8])?;
        buff.write_all(bytes)?;
        Ok(())
    }
}

impl<S: StringFamily> PackAs<Str8> for S {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let str_data = self.str_to_pack();
        let bytes = str_data.as_bytes();
        let len = bytes.len();

        if len > u8::MAX as usize {
            return Err(MPackError::size_exceeded(len, u8::MAX as usize));
        }

        buff.write_all(&[Str8::MARKER, len as u8])?;
        buff.write_all(bytes)?;
        Ok(())
    }
}

impl<S: StringFamily> PackAs<Str16> for S {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let str_data = self.str_to_pack();
        let bytes = str_data.as_bytes();
        let len = bytes.len();

        if len > u16::MAX as usize {
            return Err(MPackError::size_exceeded(len, u16::MAX as usize));
        }

        let prefix: [u8; 2] = (len as u16).to_be_bytes();
        buff.write_all(&[Str16::MARKER, prefix[0], prefix[1]])?;
        buff.write_all(bytes)?;
        Ok(())
    }
}

impl<S: StringFamily> PackAs<Str32> for S {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let str_data = self.str_to_pack();
        let bytes = str_data.as_bytes();
        let len = bytes.len();

        if len > u32::MAX as usize {
            return Err(MPackError::size_exceeded(len, u32::MAX as usize));
        }

        let prefix: [u8; 4] = (len as u32).to_be_bytes();
        buff.write_all(&[Str32::MARKER, prefix[0], prefix[1], prefix[2], prefix[3]])?;
        buff.write_all(bytes)?;
        Ok(())
    }
}
