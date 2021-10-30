use std::path::PathBuf;
use tokio::fs;
use anyhow::Result;

use crate::{
    messages::{common::*},
    util::was_not_inited_yet,
};

pub async fn list_local(app_storage_location: PathBuf) -> Result<()> {
    if was_not_inited_yet(&app_storage_location) {
        println!("{}", not_inited_yet());
        return Ok(());
    }

    let mut dir = fs::read_dir(app_storage_location.join("versions")).await?;

    while let Some(child) = dir.next_entry().await? {
        println!("{}", child.file_name().to_str().unwrap());
    }

    Ok(())
}