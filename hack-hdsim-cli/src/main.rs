use hack_hdsim_cli::{run, Opt};
use structopt::StructOpt;

fn main() {
    run(Opt::from_args());
}
