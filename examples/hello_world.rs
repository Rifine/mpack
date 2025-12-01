use std::{io::stdout};

use mpack::{MPackResult, Pack, format::{Binary, Bin8}};

fn main() -> MPackResult<()> {
    let buff = &mut stdout();
    b"Hello World!".pack::<Bin8>(buff)?;
    "Hello World!".pack::<Binary>(buff)?;
    "This is MessagePack!".pack::<String>(buff)?;
    Ok(())
}