use std::io::{Result, Write};

pub trait Pack {
    fn pack<const FORMAT: u8>(&self, buff: &mut impl Write) -> Result<()>
    where Self: PackAs<FORMAT>;
}

impl<T> Pack for T {
    fn pack<const FORMAT: u8>(&self, buff: &mut impl Write) -> Result<()>
        where Self: PackAs<FORMAT> {
            self.pack_as(buff)
    }
}

pub trait PackAs<const FORMAT: u8> {
    fn pack_as(&self, buff: &mut impl Write) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::*;

    impl PackAs<U8> for u8 {
        fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
            buff.write_all(&[U8, *self])
        }
    }

    impl PackAs<POS_FIX_INT> for u8 {
        fn pack_as(&self, buff: &mut impl Write) -> Result<()> {
            buff.write_all(&[POS_FIX_INT | (*self & 0x7f)])
        }
    }

    #[test]
    fn test_ok() {
        let v = 56u8;
        let mut buff = Vec::new();
        v.pack::<POS_FIX_INT>(&mut buff).unwrap();
        v.pack::<U8>(&mut buff).unwrap();
        println!("{buff:?}");
        assert_eq!([POS_FIX_INT | 56, U8, 56], buff[0..]);
    }
}
