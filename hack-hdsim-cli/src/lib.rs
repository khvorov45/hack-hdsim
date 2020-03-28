extern crate hack_hdsim_lib;

#[macro_use]
extern crate error_chain;

use structopt::StructOpt;

mod errors {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }
        errors { FileReadError(filepath: std::path::PathBuf) }
    }
}

pub use errors::{ErrorKind, Result, ResultExt};

/// Rust version of Nand2Tetris's hardware simulator
#[derive(StructOpt, Debug)]
pub struct Opt {
    /// .hdl file to read
    #[structopt(name = "HDLFILE", parse(from_os_str))]
    pub file: std::path::PathBuf,
}

pub fn run(opt: Opt) -> Result<()> {
    println!("Called with args\n{:#?}", opt);
    let filepath = opt.file.as_path();
    let contents = std::fs::read_to_string(filepath)
        .chain_err(|| ErrorKind::FileReadError(opt.file))?;
    hack_hdsim_lib::tokenise_hdl(contents);
    Ok(())
}
