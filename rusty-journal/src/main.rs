mod cli;
use structopt::StructOpt;

fn main() {
    cli::CommandlineArgs::from_args();
}
