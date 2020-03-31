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

pub use errors::{Error, ErrorKind, Result, ResultExt};

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
    let _contents = std::fs::read_to_string(filepath)
        .chain_err(|| ErrorKind::FileReadError(opt.file))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_fails_no_file() {
        let opt_no_file = Opt {
            file: std::path::PathBuf::from(r"./no_such_file"),
        };
        let run_no_file = run(opt_no_file);
        matches!(run_no_file, Err(_));
        let err_no_file = run_no_file.unwrap_err();
        matches!(err_no_file, Error(ErrorKind::FileReadError(_), _));
        if let ErrorKind::FileReadError(file) = err_no_file.kind() {
            assert_eq!(
                file.as_path(),
                std::path::PathBuf::from(r"./no_such_file").as_path()
            );
        }
    }
}
