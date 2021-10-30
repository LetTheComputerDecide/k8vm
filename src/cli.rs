use anyhow::{Context, Result};
use clap::{app_from_crate, App, AppSettings, Arg, ArgMatches};
use home::home_dir;

use crate::{commands, constants, messages};

pub async fn init() -> Result<()> {
    let matches = app_from_crate!()
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("deinit").about(&*messages::deinit::about().to_string()))
        .subcommand(App::new("init").about(&*messages::init::about().to_string()))
        .subcommand(
            App::new("install")
                .about(messages::install::about())
                .arg(Arg::new("version").required(true)),
        )
        .subcommand(App::new("remote").about(messages::list_remote::about()))
        .subcommand(App::new("local").about(messages::list_local::about()))
        .subcommand(
            App::new("uninstall")
                .about(&*messages::uninstall::about().to_string())
                .arg(Arg::new("version").required(true)),
        )
        .subcommand(
            App::new("use")
                .about(messages::use_version::about())
                .arg(Arg::new("version").required(true)),
        )
        .get_matches();

    let app_storage_location = home_dir()
        .context("Couldn't find the home directory.")?
        .join(constants::APP_STORAGE_DIR_NAME);

    match matches.subcommand() {
        Some(("deinit", _)) => commands::deinit(app_storage_location).await?,
        Some(("init", _)) => commands::init(app_storage_location).await?,
        Some(("install", matches)) => {
            commands::install(app_storage_location, require_version(matches)?).await?
        }
        Some(("local", _)) => commands::list_local(app_storage_location).await?,
        Some(("remote", _)) => commands::list_remote().await?,
        Some(("uninstall", matches)) => {
            commands::uninstall(app_storage_location, require_version(matches)?).await?
        }
        Some(("use", matches)) => {
            commands::use_version(app_storage_location, require_version(matches)?).await?
        }
        _ => {}
    }

    Ok(())
}

fn require_version(matches: &ArgMatches) -> Result<&str> {
    matches
        .value_of("version")
        .context("This should not happen. Version presence should be ensured by clap.")
}
