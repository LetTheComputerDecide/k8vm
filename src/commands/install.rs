use anyhow::{Context, Result};
use heim::host::{platform, Arch};
use os_info::Type::{Linux, Macos, Windows};
use std::{fs::Permissions, os::unix::prelude::PermissionsExt, path::PathBuf};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    constants,
    messages::{common::*, install::*},
    util::was_not_inited_yet,
};

pub async fn install(app_storage_location: PathBuf, version: &str) -> Result<()> {
    if was_not_inited_yet(&app_storage_location) {
        println!("{}", not_inited_yet());
        return Ok(());
    }

    let os = current_os_string().context(unsupported_os())?;
    let arch = current_arch_string()
        .await
        .context(unsupported_arch())?;
    // TODO: Check if it already exists.

    let url = constants::kubectl_download_url(version, os, arch);
    let bytes = reqwest::get(url).await?.bytes().await?;
    let mut path = app_storage_location.join("versions").join(version);

    if os == "windows" {
        path = path.join(".exe")
    }

    let mut file = File::create(path).await?;
    file.write_all(&bytes).await?;
    file.set_permissions(Permissions::from_mode(0o770)).await?;

    println!("{}", successfully_installed(version));

    Ok(())
}

fn current_os_string() -> Option<&'static str> {
    match os_info::get().os_type() {
        Windows => Some("windows"),
        Linux => Some("linux"),
        Macos => Some("darwin"),
        _ => None,
    }
}

async fn current_arch_string() -> Option<&'static str> {
    match platform().await.unwrap().architecture() {
        Arch::ARM => Some("arm"),
        Arch::AARCH64 => Some("arm64"),
        Arch::X86_64 => Some("amd64"),
        _ => None,
    }
}
