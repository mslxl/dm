use clap::{arg, ArgAction, ArgMatches, Command};
use miette::{Context, ErrReport, IntoDiagnostic, Result};
use rust_i18n::t;

use crate::{
    config,
    error::{DMError, ProfileErrorKind},
    ui::ui,
};

use super::{TomlGlobalProfileEntry, Transaction};

pub fn args() -> Command {
    Command::new("profile")
        .about(t!("profile.about"))
        .subcommand(
            Command::new("create")
                .aliases(["c", "new"])
                .about(t!("profile.create.help"))
                .arg(arg!(<NAME>).help(t!("profile.create.arg_name"))),
        )
        .subcommand(
            Command::new("use")
                .alias("u")
                .about(t!("profile.use.help"))
                .arg(arg!(<NAME>).help(t!("profile.use.arg_name"))),
        )
        .subcommand(
            Command::new("delete")
                .aliases(["d", "rm"])
                .about(t!("profile.delete.help"))
                .arg(arg!(<NAME>).help(t!("profile.delete.arg_name")))
                .arg(
                    arg!(-y - -yes)
                        .help(t!("profile.delete.arg_yes"))
                        .action(ArgAction::SetTrue),
                ),
        )
}

async fn create(matches: &ArgMatches) -> Result<()> {
    let mut transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;

    let name = matches.get_one::<String>("NAME").unwrap().clone();
    let profile_list = &mut transaction.global.registery.profile;
    if profile_list
        .iter()
        .find(|entry| entry.name == name)
        .is_some()
    {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::DuplicateCreate,
            msg: t!("error.profile.duplicate.msg", name = &name),
            advice: Some(t!("error.profile.duplicate.advice")),
        })?;
    }
    profile_list.push(TomlGlobalProfileEntry::new(name.clone()));
    transaction
        .commit()
        .wrap_err(t!("error.ctx.transcation.commit"))?;
    Ok(())
}

async fn use_profile(matches: &ArgMatches) -> Result<()> {
    let name = matches.get_one::<String>("NAME").unwrap().clone();
    let transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;
    if transaction
        .global
        .registery
        .profile
        .iter()
        .find(|entry| entry.name == name)
        .is_some()
    {
        let mut config = config::config.lock().await;
        config.using_profile = name;
        config.save().wrap_err(t!("error.ctx.config.save"))
    } else {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::NotExists,
            msg: t!("error.profile.not_exists.msg", name = &name),
            advice: None,
        })
        .into_diagnostic()
    }
}

async fn delete(matches: &ArgMatches) -> Result<()> {
    let name = matches.get_one::<String>("NAME").unwrap().clone();
    let confirm = matches.get_flag("yes");

    if name == "default" {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::IlleagalOperation,
            msg: t!("error.profile.delete_def.msg"),
            advice: Some(t!("error.profile.delete_def.advice")),
        })
        .into_diagnostic()?;
    }
    if config::config.lock().await.using_profile == name {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::IlleagalOperation,
            msg: t!("error.profile.delete_using.msg"),
            advice: Some(t!("error.profile.delete_using.advice")),
        })
        .into_diagnostic()?;
    }
    let mut transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;
    if let Some(idx) = transaction
        .global
        .registery
        .profile
        .iter()
        .position(|entry| entry.name == name)
    {
        if !confirm
            && !ui().input_yes_or_no(Some(t!("profile.delete.confirm", name = &name)), false)?
        {
            return Ok(());
        }
        transaction.global.registery.profile.remove(idx);
        transaction.commit()?;
        Ok(())
    } else {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::NotExists,
            msg: t!("error.profile.delete_not_exists.msg", name = &name),
            advice: None,
        })
        .into_diagnostic()
    }
}

async fn exec(matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("create") {
        create(matches)
            .await
            .wrap_err(t!("error.ctx.cmd.profile.create"))
    } else if let Some(matches) = matches.subcommand_matches("use") {
        use_profile(matches)
            .await
            .wrap_err(t!("error.ctx.cmd.profile.checkout"))
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        delete(matches)
            .await
            .wrap_err(t!("error.ctx.cmd.profile.delete"))
    } else {
        Ok(())
    }
}

pub async fn try_match(matches: &ArgMatches) -> Option<Result<()>> {
    Some(exec(matches.subcommand_matches("profile")?).await)
}
