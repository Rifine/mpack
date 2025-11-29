
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PackFormat {
    PositiveFixInt = 0x00,
    FixMap = 0x80,
    FixArray = 0x90,
    FixStr = 0xa0,
    Nil = 0xc0,
    // _NeverUsed = 0xc1,
    False = 0xc2,
    True,
    Bin8,       Bin16,      Bin32,
    Ext8,       Ext16,      Ext32,
    F32,        F64,
    U8,         U16,        U32,        U64,
    I8,         I16,        I32,        I64,
    FixExt1,    FixExt2,    FixExt4,    FixExt8,    FixExt16,
    Str8,       Str16,      Str32,
    Array16,    Array32,
    Map16,      Map32,
    NegativeFixInt,
}