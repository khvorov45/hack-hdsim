use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn readhdl(filepath: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Reading file {:?}", filepath);
    let contents = fs::read_to_string(filepath)?;
    println!("Read contents:\n{}", contents);
    Ok(())
}
