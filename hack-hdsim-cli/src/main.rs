use std::process;
use structopt::StructOpt;

use hack_hdsim_cli::{run, Opt};

fn main() {
    if let Err(e) = run(Opt::from_args()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
