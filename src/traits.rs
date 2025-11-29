use std::io::Write;

use crate::format::MsgPackFormat;

pub trait MsgPack {
    fn pack_format(&self) -> MsgPackFormat;
    fn pack_to(&self, buff: &mut impl std::io::Write) -> std::io::Result<()>;
    fn to_packed_bytes(&self) -> Vec<u8> {
        let mut buff: Vec<u8> = Vec::new();
        self.pack_to(&mut buff).unwrap();
        buff
    }
}