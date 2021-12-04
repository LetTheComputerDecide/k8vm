pub mod deinit {
    use colored::Colorize;
    use std::fmt::Display;

    use crate::constants;

    pub fn about() -> impl Display {
        "Delete all binaries and metadata, and revert to original kubectl."
    }

    pub fn could_not_detect_kubectl_location() -> impl Display {
        "Couldn't detect kubectl binary location. Original kubectl binary won't be restored."
            .yellow()
    }

    pub fn kubectl_restored() -> impl Display {
        format!("{} restored.", constants::KUBECTL)
    }

    pub fn successfully_deinitialized() -> impl Display {
        "Succesfully deinitialized.".green()
    }
}

pub mod init {
    use colored::Colorize;

    use crate::constants;
    use std::fmt::Display;

    pub fn about() -> impl Display {
        format!("Initialize {}.", constants::APP_NAME)
    }

    pub fn directory_already_exists(dir_name: &'static str) -> impl Display {
        format!("{} directory already exists. Skipping.", dir_name)
    }

    pub fn directory_successfuly_created(dir_name: &'static str) -> impl Display {
        format!("{} directory created in the home directory.", dir_name)
    }

    pub fn already_initialized() -> impl Display {
        format!("{} was already initialized.", constants::APP_NAME)
    }

    pub fn successfully_initialized() -> impl Display {
        format!("{} successfully initialized.", constants::APP_NAME).green()
    }
}

pub mod install {
    use std::fmt::Display;

    use colored::Colorize;

    pub fn about() -> &'static str {
        "Installs specified kubectl version(s)."
    }

    pub fn unsupported_arch() -> &'static str {
        "Your CPU architecture is not supported by kubectl."
    }

    pub fn unsupported_os() -> &'static str {
        "Your operating system is not supported by kubectl."
    }

    pub fn successfully_installed(version: &str) -> impl Display {
        format!("Successfuly installed kubectl version {}.", version).green()
    }
}

pub mod list_local {
    pub fn about() -> &'static str {
        "Lists kubectl versions installed locally."
    }
}

pub mod list_remote {
    pub fn about() -> &'static str {
        "Lists kubectl versions available remotely."
    }
}

pub mod uninstall {
    use colored::Colorize;
    use std::fmt::Display;

    pub fn about() -> impl Display {
        "Uninstalls specified kubectl version(s)."
    }

    pub fn sucessfully_uninstalled_version(version: &str) -> impl Display {
        format!("Version {} successfully uninstalled.", version).green()
    }
}

pub mod use_version {
    use std::fmt::Display;

    pub fn about() -> &'static str {
        "Select active kubectl version."
    }

    pub fn using_version(version: &str) -> impl Display {
        format!("Using version {}", version)
    }
}

pub mod common {
    use colored::Colorize;
    use std::fmt::Display;

    use crate::constants;

    pub fn not_inited_yet() -> impl Display {
        format!(
            "{} was not initialized yet. Please run: \"{} init\".",
            constants::APP_NAME,
            constants::APP_NAME
        )
        .yellow()
    }
}
