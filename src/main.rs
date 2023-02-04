mod config;
mod env;
mod error;
mod local;
mod platform;
mod task;
mod ui;

use clap::command;
use env::{get_depository_dir, get_xdg_config_dir};
use miette::Result;
use rust_i18n::t;

rust_i18n::i18n!("locales");

fn long_about() -> String {
    format!(
        "{desc}\n\n{locales_tip}\n{depository_path}\n{config_path}",
        desc = t!("app.desc"),
        locales_tip = t!(
            "app.avaliable_locales",
            locales = &format!("{:?}", available_locales())
        ),
        depository_path = t!(
            "app.depository_path",
            path = get_depository_dir().to_str().unwrap()
        ),
        config_path = t!(
            "app.configuration_path",
            path = get_xdg_config_dir().join("dm.toml").to_str().unwrap()
        )
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .name("dm")
        .about(t!("app.desc"))
        .long_about(long_about())
        .subcommand(local::profile::args())
        .subcommand(local::group::args())
        .get_matches_from(&mut wild::args_os());
    None.or(local::profile::try_match(&matches).await)
        .or(local::group::try_match(&matches).await)
        .unwrap()
}
