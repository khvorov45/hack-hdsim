use structopt::StructOpt;

use hack_hdsim_cli::{run, ErrorKind, Opt};

fn main() {
    if let Err(e) = run(Opt::from_args()) {
        match e.kind() {
            ErrorKind::FileReadError(filepath) => {
                eprintln!("Could not read '{}'", filepath.as_path().display())
            }
            _ => eprintln!("Application error: {}", e),
        }
        for e in e.iter().skip(1) {
            eprintln!("Caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace:\n{:?}", backtrace);
        }
        std::process::exit(1);
    }
}
