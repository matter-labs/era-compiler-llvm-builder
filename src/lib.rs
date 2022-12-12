//!
//! The zkEVM LLVM builder library.
//!

pub mod llvm_path;
pub mod platforms;
pub mod utils;

use std::path::PathBuf;
use std::process::Command;

use self::llvm_path::LLVMPath;

///
/// Clones the LLVM repository.
///
pub fn clone(repository_url: &str, repository_branch: &str) -> anyhow::Result<()> {
    utils::check_presence("git")?;

    let destination_path = PathBuf::from(LLVMPath::DIRECTORY_LLVM_SOURCE);
    if !destination_path.exists() {
        utils::command(
            Command::new("git").args([
                "clone",
                "--branch",
                repository_branch,
                repository_url,
                destination_path.to_string_lossy().as_ref(),
            ]),
            "LLVM repository cloning",
        )?;
    } else {
        utils::command(
            Command::new("git")
                .current_dir(destination_path.as_path())
                .args(["fetch", "--all", "--tags"]),
            "LLVM repository fetching",
        )?;
        utils::command(
            Command::new("git")
                .current_dir(destination_path.as_path())
                .args(["checkout", repository_branch]),
            "LLVM repository checking out",
        )?;
        utils::command(
            Command::new("git")
                .current_dir(destination_path.as_path())
                .args(["clean", "-d", "--force"]),
            "LLVM repository checking out",
        )?;
    }

    Ok(())
}

///
/// Executes the LLVM building.
///
pub fn build() -> anyhow::Result<()> {
    std::fs::create_dir_all(LLVMPath::DIRECTORY_LLVM_TARGET)?;

    if cfg!(target_arch = "x86_64") {
        if cfg!(target_os = "linux") {
            if cfg!(target_env = "gnu") {
                platforms::x86_64_linux_gnu::build()?;
            } else if cfg!(target_env = "musl") {
                platforms::x86_64_linux_musl::build()?;
            }
        } else if cfg!(target_os = "macos") {
            platforms::x86_64_macos::build()?;
        } else if cfg!(target_os = "windows") && cfg!(target_env = "gnu") {
            platforms::x86_64_windows_gnu::build()?;
        }
    } else if cfg!(target_arch = "aarch64") {
        if cfg!(target_os = "macos") {
            platforms::aarch64_macos::build()?;
        }
    } else {
        anyhow::bail!("Unsupported on your machine");
    }

    Ok(())
}
