use std::path::PathBuf;
use anyhow::Context;
use tokio::fs;
use which::which;
use anyhow::Result;

use crate::{
    constants,
    messages::{common::*, use_version::*},
    util::was_not_inited_yet,
};

pub async fn use_version(
    app_storage_location: PathBuf,
    version: &str,
) -> Result<()> {
    if was_not_inited_yet(&app_storage_location) {
        println!("{}", not_inited_yet());
        return Ok(());
    }
    let kubectl_location = which(constants::KUBECTL)?;
    let version_location = app_storage_location.join("versions").join(version);
    fs::metadata(&version_location).await.context(format!("Could not find version {} locally.", version))?;
    fs::remove_file(&kubectl_location).await?;
    fs::symlink(version_location, kubectl_location).await?;
    println!("{}", using_version(version));
    Ok(())
}
