use std::error::Error;
use std::fs;
use std::path::Path;

pub fn readhdl(filepath: &Path) -> Result<(), Box<dyn Error>> {
    println!("Reading file {}", filepath.display());
    let contents = fs::read_to_string(filepath)?;
    println!("Read contents:\n{}", contents);
    Ok(())
}
