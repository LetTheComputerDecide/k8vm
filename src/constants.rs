use clap::crate_name;

pub const APP_NAME: &str = crate_name!();
pub const APP_STORAGE_DIR_NAME: &str = ".k8vm";
pub const KUBECTL: &str = "kubectl";
pub fn kubectl_download_url(version: &str, os: &str, arch: &str) -> String {
    format!(
        "https://storage.googleapis.com/kubernetes-release/release/{}/bin/{}/{}/kubectl",
        version, os, arch
    )
}
