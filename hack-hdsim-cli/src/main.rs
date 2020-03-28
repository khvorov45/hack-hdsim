use structopt::StructOpt;

use hack_hdsim_cli::{run, Opt};

fn main() {
    if let Err(e) = run(Opt::from_args()) {
        eprintln!("Application error: {}", e);
        for e in e.iter().skip(1) {
            eprintln!("Caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace:\n{:?}", backtrace);
        }
        std::process::exit(1);
    }
}
