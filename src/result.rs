use crate::error::MPackError;

pub type MPackResult<T> = std::result::Result<T, MPackError>;
