use crate::{
    MPackError,
    format::{Bin8, Bin16, Bin32, Binary, MessagePackFormatInfo},
    pack::PackAs,
};

/// A family of types that can be represented as binary data in MessagePack format.
///
/// Types implementing this trait provides access to their raw byte representation,
/// which can be serialized as MessagePack binary format (bin8, bin16, bin32)
///
/// This traits allows uniform handling of different binary-like types such as
/// [`Vec<u8>`], `&[u8]`, [`String`], and `&str` for MessagePack serialization.
///
/// # Implements
/// Implements for [`PackAs<Binary>`] for all types in the [`BinaryFamily`].
///
/// This provides automatic binary serialization using the appropriate.
/// MessagePack binary format based on data length:
/// - bin8 for data up to [`u8::MAX`] bytes
/// - bin16 for data up to [`u16::MAX`] bytes
/// - bin32 for data up to [`u32::MAX`] bytes
pub trait BinaryFamily {
    /// Returns a slice containing the bytes to be packed as MessagePack binary data.
    ///
    /// # Returns
    /// A byte slice representing the data to be serialized. For string types,
    /// this return the UTF-8 encoded bytes.
    fn bytes_to_pack(&self) -> &[u8];
}

impl BinaryFamily for Vec<u8> {
    #[inline(always)]
    fn bytes_to_pack(&self) -> &[u8] {
        self
    }
}

impl BinaryFamily for [u8] {
    #[inline(always)]
    fn bytes_to_pack(&self) -> &[u8] {
        self
    }
}

impl BinaryFamily for &[u8] {
    #[inline(always)]
    fn bytes_to_pack(&self) -> &[u8] {
        self
    }
}

impl<const N: usize> BinaryFamily for [u8; N] {
    #[inline(always)]
    fn bytes_to_pack(&self) -> &[u8] {
        self
    }
}

impl<const N: usize> BinaryFamily for &[u8; N] {
    #[inline(always)]
    fn bytes_to_pack(&self) -> &[u8] {
        *self
    }
}

impl BinaryFamily for String {
    #[inline(always)]
    fn bytes_to_pack(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl BinaryFamily for str {
    #[inline(always)]
    fn bytes_to_pack(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl BinaryFamily for &str {
    #[inline(always)]
    fn bytes_to_pack(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<B: BinaryFamily> PackAs<Binary> for B {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let bytes = self.bytes_to_pack();
        let len = bytes.len();
        let prefix: &[u8] = match len {
            0xffffffff.. => return Err(MPackError::size_exceeded(len, u32::MAX as usize)),
            0xffff.. => {
                let prefix: [u8; 4] = (len as u32).to_be_bytes();
                &[Bin32::MARKER, prefix[0], prefix[1], prefix[2], prefix[3]]
            }
            0xff.. => {
                let prefix: [u8; 2] = (len as u16).to_be_bytes();
                &[Bin16::MARKER, prefix[0], prefix[1]]
            }
            _ => &[Bin8::MARKER, len as u8],
        };
        buff.write_all(&prefix)?;
        buff.write_all(bytes)?;
        Ok(())
    }
}

impl<B: BinaryFamily> PackAs<Bin8> for B {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let bytes = self.bytes_to_pack();
        let len = bytes.len();
        if len > 0xff {
            return Err(MPackError::size_exceeded(len, u8::MAX as usize));
        }
        buff.write_all(&[Bin8::MARKER, len as u8])?;
        buff.write_all(bytes)?;
        Ok(())
    }
}

impl<B: BinaryFamily> PackAs<Bin16> for B {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let bytes = self.bytes_to_pack();
        let len = bytes.len();
        if len > 0xffff {
            return Err(MPackError::size_exceeded(len, u16::MAX as usize));
        }
        let prefix: [u8; 2] = (len as u16).to_be_bytes();
        let prefix = [Bin16::MARKER, prefix[0], prefix[1]];
        buff.write_all(&prefix)?;
        buff.write_all(&bytes)?;
        Ok(())
    }
}

impl<B: BinaryFamily> PackAs<Bin32> for B {
    #[inline(always)]
    fn pack_as(&self, buff: &mut impl std::io::Write) -> crate::MPackResult<()> {
        let bytes = self.bytes_to_pack();
        let len = bytes.len();
        if len > 0xffffffff {
            return Err(MPackError::size_exceeded(len, u32::MAX as usize));
        }
        let prefix: [u8; 4] = (len as u32).to_be_bytes();
        let prefix = [Bin32::MARKER, prefix[0], prefix[1], prefix[2], prefix[3]];
        buff.write_all(&prefix)?;
        buff.write_all(&bytes)?;
        Ok(())
    }
}
