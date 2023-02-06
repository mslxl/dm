mod config;
mod env;
mod error;
mod info;
mod local;
mod platform;
mod task;
mod ui;

use clap::{command, Command};
use miette::{IntoDiagnostic, Result};
use rust_i18n::t;

rust_i18n::i18n!("locales");

async fn apply_locales() {
    let locale = &config::CONFIG.lock().await.locale;
    rust_i18n::set_locale(locale);
}

fn cli() -> Command {
    command!()
        .name("dm")
        .about(t!("app.desc"))
        .subcommand(local::profile::args())
        .subcommand(local::group::args())
        .subcommand(info::args())
        .subcommand(local::file::args_add())
}

#[tokio::main]
async fn main() -> Result<()> {
    apply_locales().await;
    let matches = cli().get_matches_from(&mut wild::args_os());
    let matched = None
        .or(local::profile::try_match(&matches).await)
        .or(local::group::try_match(&matches).await)
        .or(local::file::try_match_add(&matches).await)
        .or(info::try_match(&matches).await);
    if let None = matched {
        return cli().print_long_help().into_diagnostic();
    }else{
        return matched.unwrap();
    }
}
