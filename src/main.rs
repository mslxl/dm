mod ui;
mod cfg;
mod cmd;
mod env;
mod lock;
mod error;
mod storage;
mod checker;
mod platform;

use clap::Parser;
use cmd::Commands;


#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

fn main() {
    let args = Cli::parse_from(wild::args());
    args.command.exec();
}
