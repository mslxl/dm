mod cmd;
mod util;
mod cfg;
mod env;
mod hook;

use clap::Parser;
use cmd::{Cli, Commands};
use env::get_depository_dir;

use crate::env::{get_depository_config_filename, get_local_config_filename};

fn info() {
    println!(
        "Depositiory directory:\t{}",
        get_depository_dir().to_str().unwrap()
    );
    println!(
        "Global depositiory configuration:\t{}",
        get_depository_config_filename().to_str().unwrap()
    );
    println!(
        "Local configuration:\t{}",
        get_local_config_filename().to_str().unwrap()
    );
}

fn main() {
    let args = Cli::parse_from(wild::args());
    match args.command {
        Commands::Group { command } => cmd::group::group_commands(command),
        Commands::Info => info(),
        Commands::AddFile {
            group,
            encrypt,
            files,
        } => {
            for f in files {
                cmd::group::group_addfile(&group, encrypt, &f)
            }
        }
        _ => todo!(),
    }

    cfg::save_config();
}
