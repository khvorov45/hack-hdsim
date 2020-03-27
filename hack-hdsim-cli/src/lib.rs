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
    hack_hdsim_lib::readhdl(opt.file)
}
