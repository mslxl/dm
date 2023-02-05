mod config;
mod env;
mod error;
mod local;
mod platform;
mod task;
mod ui;

use clap::command;
use env::{get_app_config_file, get_app_data_dir};
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
            path = get_app_data_dir().unwrap().to_str().unwrap()
        ),
        config_path = t!(
            "app.configuration_path",
            path = get_app_config_file().unwrap().to_str().unwrap()
        )
    )
}

async fn apply_locales(){
    let locale = &config::CONFIG.lock().await.locale;
    rust_i18n::set_locale(locale);
}

#[tokio::main]
async fn main() -> Result<()> {
    apply_locales().await;
    let matches = command!()
        .name("dm")
        .about(t!("app.desc"))
        .long_about(long_about())
        .subcommand(local::profile::args())
        .subcommand(local::group::args())
        .subcommand(local::file::args_add())
        .get_matches_from(&mut wild::args_os());
    None.or(local::profile::try_match(&matches).await)
        .or(local::group::try_match(&matches).await)
        .or(local::file::try_match_add(&matches).await)
        .unwrap()
}
