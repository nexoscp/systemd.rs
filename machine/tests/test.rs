use machine::{Error, Machine};

#[test]
fn test() -> Result<(), Error>{
    let m = Machine::read()?;
    Ok(())
}