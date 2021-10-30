# k8vm - kubectl version manager

## Installation (Rust toolchain)
1. Clone the repository.
2. `cd` into it.
3. Run "`cargo install --path .`". This will build the binary and add it to your PATH.
4. Verify the installation by running "`k8vm`".

## Basic usage
1. Run "`k8vm init`". This is a one time setup command required to intialize the k8vm's working directories and hooking into active kubectl location.
2. Run "`k8vm remote`" to find out which k8s releases exist.
3. Use "`k8vm install v1.22.3`" to install version 1.22.3.
4. Run "`k8vm use v1.22.3`" to make it active.

### Warning
k8vm still requires some polishing, quality-of-life features, and a few edge cases are not handled yet, but it should work well for happy paths. Tested on macOS, but I would expect it to work on Linux too, and with a bit of luck, even Windows. That being said, feel free to submit issues, or even better, PRs.  :)