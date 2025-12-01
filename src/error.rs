use core::fmt;
use std::io;

pub enum MPackError {
    SizeExceeded { size: usize, limit: usize },
    InvalidUtf8(std::string::FromUtf8Error),
    InvalidUtf16(std::string::FromUtf16Error),
    IO(io::Error),
}

impl MPackError {
    #[inline(always)]
    pub fn size_exceeded(size: usize, limit: usize) -> Self {
        Self::SizeExceeded { size, limit }
    }
}

impl fmt::Display for MPackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MPackError::SizeExceeded { size, limit } => {
                write!(
                    f,
                    "mpack: error: size excedded: got {size}, expected less than {limit}"
                )
            }
            MPackError::IO(err) => {
                write!(f, "mpack: error: {err}")
            }
            MPackError::InvalidUtf8(err) => {
                write!(f, "mpack: error: {err}")
            }
            MPackError::InvalidUtf16(err) => {
                write!(f, "mpack: error: {err}")
            }
        }
    }
}

impl fmt::Debug for MPackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MPackError::SizeExceeded { size, limit } => f
                .debug_struct("InvalidSize")
                .field("size", size)
                .field("limit", limit)
                .finish(),
            MPackError::IO(err) => err.fmt(f),
            MPackError::InvalidUtf8(err) => err.fmt(f),
            MPackError::InvalidUtf16(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for MPackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MPackError::IO(err) => Some(err),
            MPackError::InvalidUtf8(err) => Some(err),
            MPackError::InvalidUtf16(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for MPackError {
    fn from(value: io::Error) -> Self {
        MPackError::IO(value)
    }
}

impl From<std::string::FromUtf8Error> for MPackError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        MPackError::InvalidUtf8(value)
    }
}

impl From<std::string::FromUtf16Error> for MPackError {
    fn from(value: std::string::FromUtf16Error) -> Self {
        MPackError::InvalidUtf16(value)
    }
}
