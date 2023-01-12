use std::path::PathBuf;

use clap::Subcommand;

use crate::{
    cfg::{
        file::{GroupFileConfigurationHelper, GroupFileConfigurationHelperMut},
        transcation::Transcation,
    },
    env::get_depository_dir,
    storage::{self, file::updatable},
    util::question,
};

#[derive(Subcommand, Clone)]
pub enum Commands {
    New {
        #[command(subcommand)]
        command: NewCommands,
    },
    AddFile {
        group: String,
        #[arg(short('l'), long, default_value_t = false)]
        hard_link: bool,
        #[arg(short('s'), long, default_value_t = false)]
        soft_link: bool,
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

fn add_file(group: String, encrypt: bool, hard_link: bool, soft_link: bool, files: Vec<String>) {
    let mut transcation = Transcation::new(get_depository_dir());

    {
        let mut group = transcation.group_mut(group).unwrap();
        for file in files {
            let path = PathBuf::from(&file);
            if !path.exists() {
                panic!("File be must exists: {}", file);
            }
            let mut helper = group.add_file(path).unwrap();
            helper.set_encrypt(encrypt);
            helper.set_hard_link(hard_link);
            helper.set_soft_link(soft_link);

            storage::file::update_file(&helper).unwrap();
        }
    }

    transcation.save().unwrap();
}

fn update_group(groups: Vec<String>) {
    let transcation = Transcation::new(get_depository_dir());

    let mut update_all_group = false;
    for group in groups {
        let group = transcation.group(group).unwrap();
        let files = group.files();

        let mut update_all_file = update_all_group;
        for f in files {
            if updatable(&f).unwrap() {
                if !update_all_file {
                    let g = question(
                        &format!("Update {}?", f.get_local_path().unwrap()),
                        &[
                            ('Y', "Update this file"),
                            ('N', "Skip this file"),
                            ('A', "Update all file in same group"),
                            ('G', "Update all file"),
                        ],
                    );
                    if g == 'N' {
                        println!("Skipped");
                        continue;
                    } else if g == 'A' {
                        update_all_file = true;
                    } else if g == 'G' {
                        update_all_group = true;
                    }
                }
                // The action will be skiped if user input 'N' above
                storage::file::update_file(&f).unwrap();
                println!("Succeed")
            }
        }
    }
}

impl Commands {
    pub fn exec(self) {
        match self {
            Commands::New { command } => command.exec(),
            Commands::AddDir {
                group,
                encrypt,
                files,
            } => todo!(),
            Commands::AddFile {
                group,
                encrypt,
                hard_link,
                soft_link,
                files,
            } => add_file(group, encrypt, hard_link, soft_link, files),
            Commands::Remove { files } => todo!(),
            Commands::Update { group } => update_group(group),
            Commands::Install { group } => todo!(),
            Commands::Push => todo!(),
            Commands::Pull => todo!(),
            Commands::Tui => todo!(),
            Commands::Config { key, value, local } => todo!(),
            Commands::Info => info(),
        }
    }
}
