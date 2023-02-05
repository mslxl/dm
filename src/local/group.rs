use clap::{arg, ArgAction, ArgMatches, Command};
use miette::{Context, Result};
use rust_i18n::t;

use crate::config::{self, config};

use super::Transaction;

pub fn args() -> Command {
    Command::new("group").about(t!("group.about")).subcommand(
        Command::new("create")
            .alias("c")
            .about(t!("group.create.help"))
            .arg(arg!(<NAME>).help(t!("group.create.arg_name")))
            .arg(
                arg!(-n --nouse)
                    .help(t!("group.create.arg_nouse"))
                    .action(ArgAction::SetTrue),
            ),
    )
}

async fn create(matches: &ArgMatches) -> Result<()> {
    let name = matches.get_one::<String>("NAME").unwrap().clone();
    let no_use = matches.get_flag("nouse");
    let mut transaction = Transaction::start()?;
    transaction.create_group(&name)?;

    if !no_use {
        let use_profile = &config::config.lock().await.using_profile;
        let pos = transaction
            .global
            .registery
            .profile
            .iter()
            .position(|entry| &entry.name == use_profile)
            .unwrap();
        transaction
            .global
            .registery
            .profile
            .get_mut(pos)
            .unwrap()
            .group
            .push(name);
    }

    transaction.commit()
}

async fn exec(matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("create") {
        create(matches)
            .await
            .wrap_err(t!("error.ctx.cmd.group.create"))
    } else {
        Ok(())
    }
}

pub async fn try_match(matches: &ArgMatches) -> Option<Result<()>> {
    Some(exec(matches.subcommand_matches("group")?).await)
}
