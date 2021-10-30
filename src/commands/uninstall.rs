use std::path::{PathBuf};
use tokio::fs;
use anyhow::{Context, Result};

use crate::{
    messages::{common::*, uninstall::*},
    util::was_not_inited_yet,
};

pub async fn uninstall(app_storage_location: PathBuf, version: &str) -> Result<()> {
    if was_not_inited_yet(&app_storage_location) {
        println!("{}", not_inited_yet());
        return Ok(());
    }
    let file = app_storage_location.join("versions").join(version);
    fs::remove_file(file).await.context(format!("Could not find version {} locally.", version))?;
    println!("{}", sucessfully_uninstalled_version(version));
    Ok(())
}