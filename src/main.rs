mod uicli;

use dm::config;
use miette::{IntoDiagnostic, Result};
rust_i18n::i18n!("locales");

async fn apply_locales() {
    let locale = &config::CONFIG.lock().await.locale;
    rust_i18n::set_locale(locale);
}

mod cli {
    use clap::{command, Command};
    use rust_i18n::t;

    pub mod local {
        pub mod profile {
            use clap::{arg, ArgAction, ArgMatches, Command};
            use miette::{Context, Result};
            use rust_i18n::t;

            use crate::uicli;

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

            async fn exec(matches: &ArgMatches) -> Result<()> {
                if let Some(matches) = matches.subcommand_matches("create") {
                    let name = matches.get_one::<String>("NAME").unwrap().clone();
                    dm::local::profile::create_profile(name)
                        .await
                        .wrap_err(t!("error.ctx.cmd.profile.create"))
                } else if let Some(matches) = matches.subcommand_matches("use") {
                    let name = matches.get_one::<String>("NAME").unwrap().clone();
                    dm::local::profile::use_profile(name)
                        .await
                        .wrap_err(t!("error.ctx.cmd.profile.checkout"))
                } else if let Some(matches) = matches.subcommand_matches("delete") {
                    let name = matches.get_one::<String>("NAME").unwrap().clone();
                    let confirm = matches.get_flag("yes");
                    dm::local::profile::delete(&uicli::Cli, name, confirm)
                        .await
                        .wrap_err(t!("error.ctx.cmd.profile.delete"))
                } else {
                    Ok(())
                }
            }

            pub async fn try_match(matches: &ArgMatches) -> Option<Result<()>> {
                Some(exec(matches.subcommand_matches("profile")?).await)
            }
        }
        pub mod group {
            use clap::{arg, ArgAction, ArgMatches, Command};
            use miette::{Context, Result};
            use rust_i18n::t;

            pub fn args() -> Command {
                Command::new("group").about(t!("group.about")).subcommand(
                    Command::new("create")
                        .alias("c")
                        .about(t!("group.create.help"))
                        .arg(arg!(<NAME>).help(t!("group.create.arg_name")))
                        .arg(
                            arg!(-n - -nouse)
                                .help(t!("group.create.arg_nouse"))
                                .action(ArgAction::SetTrue),
                        ),
                )
            }

            async fn exec(matches: &ArgMatches) -> Result<()> {
                if let Some(matches) = matches.subcommand_matches("create") {
                    let name = matches.get_one::<String>("NAME").unwrap().clone();
                    let no_use = matches.get_flag("nouse");
                    dm::local::group::create_group(name, no_use)
                        .await
                        .wrap_err(t!("error.ctx.cmd.group.create"))
                } else {
                    Ok(())
                }
            }

            pub async fn try_match(matches: &ArgMatches) -> Option<Result<()>> {
                Some(exec(matches.subcommand_matches("group")?).await)
            }
        }
        pub mod file {

            use std::path::PathBuf;

            use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
            use miette::{Context, Result};
            use rust_i18n::t;

            use crate::uicli;

            async fn exec_add(matches: &ArgMatches) -> Result<()> {
                let path = matches.get_one::<PathBuf>("PATH").unwrap();
                let group_name = matches.get_one::<String>("GROUP").unwrap();
                let try_recongize = matches.get_flag("recongize");
                let manual_install = matches.get_flag("manual");

                dm::local::file::add_file(
                    &uicli::Cli,
                    path,
                    group_name,
                    try_recongize,
                    manual_install,
                )
                .await
            }
            pub async fn try_match_add(matches: &ArgMatches) -> Option<Result<()>> {
                Some(
                    exec_add(matches.subcommand_matches("add")?)
                        .await
                        .wrap_err(t!("error.ctx.cmd.add")),
                )
            }

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
        }
    }

    pub mod info {
        use clap::{ArgMatches, Command};
        use miette::{Context, Result};
        use rust_i18n::t;

        pub fn args() -> Command {
            Command::new("info").alias("i").about(t!("info.help"))
        }

        async fn exec(_matches: &ArgMatches) -> Result<()> {
            println!("{}", dm::info::all_info()?);
            Ok(())
        }

        pub async fn try_match(matches: &ArgMatches) -> Option<Result<()>> {
            Some(
                exec(matches.subcommand_matches("info")?)
                    .await
                    .wrap_err(t!("error.ctx.cmd.info")),
            )
        }
    }
    pub fn args() -> Command {
        command!()
            .name("dm")
            .about(t!("app.desc"))
            .subcommand(crate::cli::local::profile::args())
            .subcommand(crate::cli::local::group::args())
            .subcommand(crate::cli::info::args())
            .subcommand(crate::cli::local::file::args_add())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    apply_locales().await;
    let matches = cli::args().get_matches_from(&mut wild::args_os());
    let matched = None
        .or(cli::local::profile::try_match(&matches).await)
        .or(cli::local::group::try_match(&matches).await)
        .or(cli::local::file::try_match_add(&matches).await)
        .or(cli::info::try_match(&matches).await);
    if let None = matched {
        return cli::args().print_long_help().into_diagnostic();
    } else {
        return matched.unwrap();
    }
}
