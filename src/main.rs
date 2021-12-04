use colored::Colorize;
mod commands {
    mod deinit;
    pub use deinit::deinit;

    mod init;
    pub use init::init;

    mod install;
    pub use install::install;

    mod list_local;
    pub use list_local::list_local;

    mod list_remote;
    pub use list_remote::list_remote;

    mod uninstall;
    pub use uninstall::uninstall;

    mod use_version;
    pub use use_version::use_version;
}
mod cli;
mod constants;
mod messages;
mod util;

#[tokio::main]
async fn main() {
    human_panic::setup_panic!();

    if let Err(err) = cli::init().await {
        eprintln!("{}", format!("{}", err).red());
    }
}
