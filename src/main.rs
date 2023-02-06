mod config;
mod env;
mod error;
mod info;
mod local;
mod platform;
mod task;
mod ui;

use clap::command;
use miette::Result;
use rust_i18n::t;

rust_i18n::i18n!("locales");

async fn apply_locales() {
    let locale = &config::CONFIG.lock().await.locale;
    rust_i18n::set_locale(locale);
}

#[tokio::main]
async fn main() -> Result<()> {
    apply_locales().await;
    let matches = command!()
        .name("dm")
        .about(t!("app.desc"))
        .subcommand(local::profile::args())
        .subcommand(local::group::args())
        .subcommand(info::args())
        .subcommand(local::file::args_add())
        .get_matches_from(&mut wild::args_os());
    None.or(local::profile::try_match(&matches).await)
        .or(local::group::try_match(&matches).await)
        .or(local::file::try_match_add(&matches).await)
        .or(info::try_match(&matches).await)
        .unwrap()
}
