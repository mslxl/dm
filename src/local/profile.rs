use clap::{arg, ArgMatches, Command};
use miette::{Context, IntoDiagnostic, Result, ErrReport};
use rust_i18n::t;

use super::{TomlGlobalProfileEntry, Transcation};

pub fn args() -> Command {
    Command::new("profile")
        .about(t!("profile.about"))
        .subcommand(
            Command::new("create")
                .alias("c")
                .about(t!("profile.create.help"))
                .arg(arg!(<NAME>).help(t!("profile.create.arg_name"))),
        )
}

async fn create(matches: &ArgMatches) -> Result<()> {
    let mut transaction = Transcation::start().wrap_err(t!("error.ctx.cmd.profile.create"))?;
    let name = matches.get_one::<String>("NAME").unwrap();
    transaction
        .global
        .general
        .profile
        .push(TomlGlobalProfileEntry::new(name.clone()));
    transaction
        .commit()
        .wrap_err(t!("error.ctx.cmd.profile.create"))?;
    Ok(())
}

async fn exec(matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("create") {
        create(matches).await
    } else {
        Ok(())
    }
}

pub async fn try_match(matches: &ArgMatches) -> Option<Result<()>> {
    Some(exec(matches.subcommand_matches("profile")?).await)
}
