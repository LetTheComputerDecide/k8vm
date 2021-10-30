use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;
use which::which;

use crate::{
    constants,
    messages::{common::*, deinit::*},
    util::was_not_inited_yet,
};

pub async fn deinit(app_storage_location: PathBuf) -> Result<()> {
    if was_not_inited_yet(&app_storage_location) {
        println!("{}", not_inited_yet());
        return Ok(());
    }
    
    match which(constants::KUBECTL) {
        Ok(kubectl_location) => {
            let original_kubectl_location = app_storage_location
                .join("original")
                .join(constants::KUBECTL);
            fs::remove_file(&kubectl_location).await?;
            fs::rename(&original_kubectl_location, &kubectl_location).await?;
            println!("{}", kubectl_restored());
        }
        Err(_) => {
            println!("{}", could_not_detect_kubectl_location());
        }
    }
    
    fs::remove_dir_all(app_storage_location).await?;
    println!("{}", successfully_deinitialized());
    
    Ok(())
}
