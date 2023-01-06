mod cfg;
mod cmd;
mod env;
mod util;

use clap::Parser;
use cmd::{start_dm, Commands};
use env::get_depository_dir;


#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

fn main() {
    let args = Cli::parse_from(wild::args());
    start_dm(args.command);
}
