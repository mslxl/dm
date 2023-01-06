use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{
    cfg::transcation::{self, Transcation},
    env::get_depository_dir,
};

#[derive(Subcommand, Clone)]
pub enum Commands {
    New {
        #[command(subcommand)]
        command: NewCommands,
    },
    AddFile {
        group: String,
        #[arg(short, long, default_value_t = false)]
        encrypt: bool,
        files: Vec<String>,
    },
    AddDir {
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
    Install {
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
pub enum NewCommands {
    Group { name: String, desc: Option<String> },
}

impl NewCommands {
    pub fn exec(self) {
        match self {
            NewCommands::Group { name, desc } => {
                let mut transcation = Transcation::new(get_depository_dir());
                transcation.new_group(name.clone()).unwrap();
                if let Some(desc) = desc {
                    transcation.group_mut(name).unwrap().set_desc(&desc);
                }
                transcation.save().unwrap();
            }
        }
    }
}

fn info() {
    println!(
        "Depositiory directory:\t{}",
        get_depository_dir().to_str().unwrap()
    );
}
pub fn start_dm(args: Commands) {
    match args {
        Commands::New { command } => command.exec(),
        Commands::AddFile {
            group,
            encrypt,
            files,
        } => todo!(),
        Commands::AddDir {
            group,
            encrypt,
            files,
        } => todo!(),
        Commands::Remove { files } => todo!(),
        Commands::Update { group } => todo!(),
        Commands::Install { group } => todo!(),
        Commands::Push => todo!(),
        Commands::Pull => todo!(),
        Commands::Tui => todo!(),
        Commands::Config { key, value, local } => todo!(),
        Commands::Info => info(),
    }
}
