use std::path::PathBuf;

pub fn was_already_inited(app_storage_location: &PathBuf) -> bool {
    app_storage_location.exists()
}

pub fn was_not_inited_yet(app_storage_location: &PathBuf) -> bool {
    !was_already_inited(app_storage_location)
}
