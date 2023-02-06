use std::path::PathBuf;

use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use miette::{Context, IntoDiagnostic, Result};
use rust_i18n::t;

use crate::error::{DMError, GroupErrorKind};

use super::Transaction;

pub fn args_add() -> Command {
    Command::new("add")
        .alias("a")
        .about(t!("file.add.help"))
        .arg(arg!(<GROUP>).help(t!("file.add.arg_name")))
        .arg(
            arg!(<PATH>)
                .help(t!("file.add.arg_path"))
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-c - -compress)
                .help(t!("file.add.arg_compress"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-s - -symbolic)
                .help(t!("file.add.arg_symbolic_link"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-l - -link)
                .help(t!("file.add.arg_link"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-e - -encrypt)
                .help(t!("file.add.arg_encrypt"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-m - -manual)
                .help(t!("file.add.arg_manual_install"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-r - -recongize)
                .help(t!("file.add.arg_recongize"))
                .action(ArgAction::SetTrue),
        )
}

async fn exec_add(matches: &ArgMatches) -> Result<()> {
    let path = matches.get_one::<PathBuf>("PATH").unwrap();
    let group_name = matches.get_one::<String>("GROUP").unwrap();
    let mut transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;
    let group = transaction
        .group_mut(group_name)?
        .ok_or(DMError::GroupError {
            kind: GroupErrorKind::NotExists,
            msg: t!("error.group.not_exists", name = group_name),
            advice: None,
        })
        .into_diagnostic()?;

    Ok(())
}

pub async fn try_match_add(matches: &ArgMatches) -> Option<Result<()>> {
    Some(
        exec_add(matches.subcommand_matches("add")?)
            .await
            .wrap_err(t!("error.ctx.cmd.add")),
    )
}
