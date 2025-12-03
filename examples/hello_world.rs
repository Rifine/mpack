use mpack::{MPackResult, Pack, format::{Binary, Bin8}};
use hex;

fn main() -> MPackResult<()> {
    let buff = &mut Vec::new();
    42.pack::<u8>(buff)?;
    42.pack::<u16>(buff)?;
    42.pack::<u32>(buff)?;
    42.pack::<u64>(buff)?;
    42.pack::<i8>(buff)?;
    42.pack::<i16>(buff)?;
    42.pack::<i32>(buff)?;
    42.pack::<i64>(buff)?;
    42.1.pack::<f32>(buff)?;
    42.2.pack::<f64>(buff)?;
    b"Hello World!".pack::<Bin8>(buff)?;
    "Hello World!".pack::<Binary>(buff)?;
    "This is MessagePack!\n".pack::<String>(buff)?;
    let result = hex::encode(buff);
    println!("{result}");
    Ok(())
}