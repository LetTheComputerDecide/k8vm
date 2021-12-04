use std::path::Path;

pub fn was_already_inited(app_storage_location: &Path) -> bool {
    app_storage_location.exists()
}

pub fn was_not_inited_yet(app_storage_location: &Path) -> bool {
    !was_already_inited(app_storage_location)
}
