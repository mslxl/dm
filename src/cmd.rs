use std::{path::PathBuf, process::Stdio};

use clap::Subcommand;

use crate::{
    cfg::{
        file::{GroupFileConfigurationHelper, GroupFileConfigurationHelperMut},
        transaction::Transaction,
    },
    env::get_depository_dir,
    storage::{install_file, is_file_updatable, update_file},
    ui,
};

#[derive(Subcommand, Clone)]
pub enum Commands {
    New {
        #[command(subcommand)]
        command: NewCommands,
    },
    Add {
        /// Target group name
        group: String,

        #[arg(short('l'), long, default_value_t = false)]
        /// Create hard-link instead of copy, can't apply to floder
        hard_link: bool,

        #[arg(short('s'), long, default_value_t = false)]
        /// Create soft-link instead of copy
        soft_link: bool,

        #[arg(short, long, value_name="GPG recipient")]
        /// Use specify recipient to encrypt file
        encrypt: Option<String>,

        #[arg(short, long, default_value_t = false)]
        /// Use zstd compress file
        compress: bool,
        #[clap(required = true)]
        file: Vec<String>,
        #[arg(long)]
        /// Exclude files in folder by glob expression
        exclude: Vec<String>,
    },
    Rm {
        #[clap(required = true)]
        files: Vec<String>,
    },
    Update {
        group: Vec<String>,
    },
    Install {
        group: Vec<String>,
    },
    Check {
        group: Option<Vec<String>>,
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

impl Commands {
    pub fn exec(self) {
        match self {
            Commands::New { command } => command.exec(),
            Commands::Add {
                group,
                encrypt,
                compress,
                hard_link,
                soft_link,
                file,
                exclude,
            } => cmd_add_file(
                group, compress, encrypt, hard_link, soft_link, file, exclude,
            ),
            Commands::Rm { files } => todo!(),
            Commands::Update { group } => cmd_update_group(group),
            Commands::Install { group } => cmd_install(group),
            Commands::Push => todo!(),
            Commands::Pull => todo!(),
            Commands::Tui => todo!(),
            Commands::Config { key, value, local } => todo!(),
            Commands::Info => cmd_info(),
            Commands::Check { group } => cmd_health_check(group),
        }
    }
}

#[derive(Subcommand, Clone)]
pub enum NewCommands {
    Group { name: String, desc: Option<String> },
}
impl NewCommands {
    pub fn exec(self) {
        match self {
            NewCommands::Group { name, desc } => {
                let mut transcation = Transaction::new(get_depository_dir());
                transcation.new_group(name.clone()).unwrap();
                if let Some(desc) = desc {
                    transcation.group_mut(name).unwrap().set_desc(&desc);
                }
                transcation.save().unwrap();
            }
        }
    }
}

fn cmd_info() {
    println!(
        "Depositiory directory:\t{}",
        get_depository_dir().to_str().unwrap()
    );
}

fn cmd_install(groups: Vec<String>) {
    let transcation = Transaction::new(get_depository_dir());

    for group in groups {
        let config = transcation.group(group.clone());
        match config {
            None => panic!("Group {} not exists", group),
            Some(config) => config.files().for_each(|file| {
                install_file(&file).unwrap();
            }),
        }
    }
}
fn cmd_add_file(
    group: String,
    compress: bool,
    encrypt: Option<String>,
    hard_link: bool,
    soft_link: bool,
    files: Vec<String>,
    exculde: Vec<String>,
) {
    let mut transcation = Transaction::new(get_depository_dir());

    {
        let mut group = transcation.group_mut(group).unwrap();
        for file in files {
            let path = PathBuf::from(&file);
            if !path.exists() {
                panic!("File be must exists: {}", file);
            }
            if path.is_file() {
                if !exculde.is_empty() {
                    panic!("Can't specify exculde option when adding file")
                }
                let mut helper = group.add_file(path).unwrap();
                if let Some(ref recipient) = encrypt {
                    helper.set_encrypt(true);
                    helper.set_encrypt_recipient(recipient);
                }else{
                    helper.set_encrypt(false);
                }
                helper.set_hard_link(hard_link);
                helper.set_soft_link(soft_link);
                helper.set_compress(compress);
                if compress {
                    helper.set_depository_path(&format!(
                        "{}.zst",
                        helper.get_depository_path().unwrap()
                    ));
                }
                if let Some(_) = encrypt {
                    helper.set_depository_path(&format!(
                        "{}.encrypt",
                        helper.get_depository_path().unwrap()
                    ));
                }

                update_file(&helper).unwrap();
            } else {

            }
        }
    }

    transcation.save().unwrap();
}

fn cmd_update_group(groups: Vec<String>) {
    let transcation = Transaction::new(get_depository_dir());

    let mut update_all_group = false;
    for group in groups {
        let group = transcation.group(group).unwrap();
        let files = group.files();

        let mut update_all_file = update_all_group;
        for f in files {
            let path = PathBuf::from(f.get_local_path().unwrap());
            if !path.exists(){
                todo!("File has been deleted! Add remove from depository feature")
            }

            if is_file_updatable(&f).unwrap() {
                if !update_all_file {
                    match ui::get_ui().select(
                        &format!("Update {}?", f.get_local_path().unwrap()),
                        &[
                            ('Y', "Update this file"),
                            ('N', "Skip this file"),
                            ('A', "Update all file in same group"),
                            ('G', "Update all file"),
                        ],
                    ) {
                        'N' => {
                            println!("Skipped");
                            continue;
                        }
                        'A' => {
                            update_all_file = true;
                        }
                        'G' => {
                            update_all_group = true;
                        }
                        'Y' => {}
                        _ => unreachable!(),
                    }
                }
                // The action will be skiped if user input 'N' above
                update_file(&f).unwrap();
                println!("Succeed")
            }
        }
    }
}

fn cmd_health_check(group: Option<Vec<String>>) {
    std::process::Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Missing git program, try to install it or add it to PATH");
    std::process::Command::new("gpg")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Missing gpg program, try to install it or add it to PATH");
    let name = {
        let transcation = Transaction::new(get_depository_dir());

        if let Some(names) = group {
            if names.is_empty() {
                transcation.groups().map(ToString::to_string).collect()
            } else {
                names
            }
        } else {
            transcation.groups().map(ToString::to_string).collect()
        }
    };

    for name in name {
        todo!()
    }
}
