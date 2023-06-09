//!
//! The zkEVM LLVM builder library.
//!

pub mod build_type;
pub mod llvm_path;
pub mod lock;
pub mod platforms;
pub mod utils;

pub use self::build_type::BuildType;
pub use self::llvm_path::LLVMPath;
pub use self::lock::Lock;

use std::path::PathBuf;
use std::process::Command;

///
/// Executes the LLVM repository cloning.
///
pub fn clone(lock: Lock) -> anyhow::Result<()> {
    utils::check_presence("git")?;

    let destination_path = PathBuf::from(LLVMPath::DIRECTORY_LLVM_SOURCE);
    if destination_path.exists() {
        anyhow::bail!(
            "The repository is already cloned at {:?}. Use `checkout` instead",
            destination_path
        );
    }

    utils::command(
        Command::new("git").args([
            "clone",
            "--branch",
            lock.branch.as_str(),
            lock.url.as_str(),
            destination_path.to_string_lossy().as_ref(),
        ]),
        "LLVM repository cloning",
    )?;

    if let Some(r#ref) = lock.r#ref {
        utils::command(
            Command::new("git").args(["checkout", r#ref.as_str()]),
            "LLVM repository commit checking out",
        )?;
    }

    Ok(())
}

///
/// Executes the checkout of the specified branch.
///
pub fn checkout(lock: Lock, force: bool) -> anyhow::Result<()> {
    let destination_path = PathBuf::from(LLVMPath::DIRECTORY_LLVM_SOURCE);

    utils::command(
        Command::new("git")
            .current_dir(destination_path.as_path())
            .args(["fetch", "--all", "--tags"]),
        "LLVM repository data fetching",
    )?;

    if force {
        utils::command(
            Command::new("git")
                .current_dir(destination_path.as_path())
                .args(["clean", "-d", "-x", "--force"]),
            "LLVM repository cleaning",
        )?;
    }

    utils::command(
        Command::new("git")
            .current_dir(destination_path.as_path())
            .args(["checkout", "--force", lock.branch.as_str()]),
        "LLVM repository data pulling",
    )?;

    if let Some(r#ref) = lock.r#ref {
        let mut checkout_command = Command::new("git");
        checkout_command.current_dir(destination_path.as_path());
        checkout_command.arg("checkout");
        if force {
            checkout_command.arg("--force");
        }
        checkout_command.arg(r#ref);
        utils::command(&mut checkout_command, "LLVM repository checking out")?;
    }

    Ok(())
}

///
/// Executes the building of the LLVM framework for the platform determined by the cfg macro.
/// Since cfg is evaluated at compile time, overriding the platform with a command-line
/// argument is not possible. So for cross-platform testing, comment out all but the
/// line to be tested, and perhaps also checks in the platform-specific build method.
///
pub fn build(
    build_type: BuildType,
    enable_tests: bool,
    enable_coverage: bool,
    extra_args: Vec<String>,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(LLVMPath::DIRECTORY_LLVM_TARGET)?;

    if cfg!(target_arch = "x86_64") {
        if cfg!(target_os = "linux") {
            if cfg!(target_env = "gnu") {
                platforms::x86_64_linux_gnu::build(
                    build_type,
                    enable_tests,
                    enable_coverage,
                    extra_args,
                )?;
            } else if cfg!(target_env = "musl") {
                platforms::x86_64_linux_musl::build(
                    build_type,
                    enable_tests,
                    enable_coverage,
                    extra_args,
                )?;
            }
        } else if cfg!(target_os = "macos") {
            platforms::x86_64_macos::build(build_type, enable_tests, enable_coverage, extra_args)?;
        } else if cfg!(target_os = "windows") && cfg!(target_env = "gnu") {
            platforms::x86_64_windows_gnu::build(
                build_type,
                enable_tests,
                enable_coverage,
                extra_args,
            )?;
        }
    } else if cfg!(target_arch = "aarch64") {
        if cfg!(target_os = "macos") {
            platforms::aarch64_macos::build(build_type, enable_tests, enable_coverage, extra_args)?;
        }
    } else {
        anyhow::bail!("Unsupported on your machine");
    }

    Ok(())
}

///
/// Executes the build artifacts cleaning.
///
pub fn clean() -> anyhow::Result<()> {
    Ok(std::fs::remove_dir_all(PathBuf::from(
        LLVMPath::DIRECTORY_LLVM_TARGET,
    ))?)
}
