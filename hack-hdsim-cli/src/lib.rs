extern crate hack_hdsim_lib;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

/// Rust version of Nand2Tetris's hardware simulator
#[derive(StructOpt, Debug)]
pub struct Opt {
    /// .hdl file to read
    #[structopt(name = "HDLFILE", parse(from_os_str))]
    pub file: PathBuf,
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    println!("Called with args\n{:#?}", opt);
    let filepath = opt.file.as_path();
    let contents = fs::read_to_string(filepath)?;
    hack_hdsim_lib::tokenise_hdl(contents);
    Ok(())
}
