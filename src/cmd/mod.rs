use clap::{Parser, Subcommand};

pub mod group;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Group {
        #[command(subcommand)]
        command: GroupCommands,
    },
    AddFile {
        group: String,
        #[arg(short, long, default_value_t = false)]
        encrypt: bool,
        files: Vec<String>,
    },
    AddDir{
        group: String,
        #[arg(short, long, default_value_t = false)]
        encrypt: bool,
        files: Vec<String>,
    },
    Remove {
        files: Vec<String>,
    },
    Update {
        group: Vec<String>,
    },
    Install{
        group: Vec<String>,
    },
    Push,
    Pull,
    Tui,
    Config {
        key: String,
        value: String,
        #[arg(long, default_value_t = false)]
        local: bool,
    },
    Info,
}

#[derive(Subcommand, Clone)]
pub enum GroupCommands {
    New { name: String, desc: Option<String> },
    Rm { name: String },
    Disable { name: String },
    Enable { name: String },
    Config { key: String, value: String },
}
