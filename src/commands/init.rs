use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::{fs, try_join};
use which::which;

use crate::{constants, messages::init::*, util::was_already_inited};

pub async fn init(app_storage_location: PathBuf) -> Result<()> {
    if was_already_inited(&app_storage_location) {
        println!("{}", already_initialized());
        return Ok(());
    }
    
    fs::create_dir(&app_storage_location)
        .await
        .context(directory_already_exists(constants::APP_STORAGE_DIR_NAME))?;

    let versions_location = app_storage_location.join("versions");
    let original_location = app_storage_location.join("original");

    try_join!(
        fs::create_dir(versions_location),
        fs::create_dir(&original_location)
    )?;

    println!("{}", directory_successfuly_created(
        constants::APP_STORAGE_DIR_NAME
    ));

    let kubectl_location = which(constants::KUBECTL)?;
    let original_kubectl_location = original_location.join(constants::KUBECTL);

    if original_kubectl_location.exists() {
        println!("Original kubectl already replaced. Skipping.");
    } else {
        println!(
            "Found {} at {}.",
            constants::KUBECTL,
            kubectl_location.to_str().unwrap()
        );

        fs::rename(&kubectl_location, &original_kubectl_location).await?;
        println!("Moved {}.", constants::KUBECTL);
        fs::symlink(original_kubectl_location, kubectl_location).await?;
        println!("Symlinked back.");
    }

    println!("{}", successfully_initialized());

    Ok(())
}
