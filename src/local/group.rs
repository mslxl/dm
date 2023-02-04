use clap::{arg, ArgMatches, Command};
use rust_i18n::t;
use miette::Result;


pub fn args() -> Command {
    Command::new("group").about(t!("group.about")).subcommand(
        Command::new("create")
            .alias("c")
            .about(t!("group.create.help"))
            .arg(arg!(<NAME>).help(t!("group.create.arg_name"))),
    )
}

async fn exec(_matches: &ArgMatches) -> Result<()> {
    todo!()
}

pub async fn try_match(matches: &ArgMatches) -> Option<Result<()>> {
    Some(exec(matches.subcommand_matches("group")?).await)
}
