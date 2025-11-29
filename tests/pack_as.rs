use mpack::{pack::Pack};

#[test]
fn pack_u8_as_u16() {
    let mut buff = Vec::new();
    let v = 5u8;
    v.pack_as::<u16>(&mut buff).unwrap();
    assert_eq!([0xcd, 0, 5], *buff);
}

#[test]
fn pack_bool_as_str() {
    let mut buff = Vec::new();
    let v = false;
    v.pack_as::<String>(&mut buff).unwrap();
    assert_eq!([0xa5, b'f', b'a', b'l', b's', b'e'], *buff);
}