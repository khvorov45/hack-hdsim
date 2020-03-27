extern crate hack_hdsim_lib;

use std::path::PathBuf;
use structopt::StructOpt;

/// Rust version of Nano2Tetris's hardware simulator
#[derive(StructOpt, Debug)]
pub struct Opt {
    /// .hdl file to read
    #[structopt(name = "HDLFILE", parse(from_os_str))]
    file: PathBuf,
}

pub fn run(opt: Opt) {
    println!("Called with args\n{:#?}", opt);
    let filepath = opt.file.as_path();
    match hack_hdsim_lib::readhdl(filepath) {
        Ok(()) => println!("Read successfully"),
        Err(e) => eprintln!("Failed to read {}: {}", filepath.display(), e),
    };
}
