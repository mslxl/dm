use miette::{Context, Result};

use clap::{ArgMatches, Command};
use rust_i18n::t;

use crate::{
    available_locales,
    env::{self, get_app_config_file, get_app_data_dir},
};

pub fn args() -> Command {
    Command::new("info").alias("i").about(t!("info.help"))
}

async fn exec(_matches: &ArgMatches) -> Result<()> {
    println!("{}", all_info()?);
    Ok(())
}

pub async fn try_match(matches: &ArgMatches) -> Option<Result<()>> {
    Some(
        exec(matches.subcommand_matches("info")?)
            .await
            .wrap_err(t!("error.ctx.cmd.info")),
    )
}

fn all_info() -> Result<String> {
    Ok(format!(
        "{locales_tip}\n{depository_path}\n{config_path}\n{pssl}",
        locales_tip = t!(
            "app.avaliable_locales",
            locales = &format!("{:?}", available_locales())
        ),
        depository_path = t!(
            "app.depository_path",
            path = get_app_data_dir().unwrap().to_str().unwrap()
        ),
        config_path = t!(
            "app.configuration_path",
            path = get_app_config_file().unwrap().to_str().unwrap()
        ),
        pssl = t!(
            "app.info.pssl",
            loc = &format!("\n{}", env::SpecDir::new()?.display_tree())
                .lines()
                .map(|l| format!("\t{}", l))
                .reduce(|a, b| format!("{}\n{}", a, b))
                .unwrap()
        )
    ))
}
