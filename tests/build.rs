use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;

mod constants;

/// Tests building without cloning LLVM repository.
///
/// This test verifies that the build process fails when attempting to build LLVM without
/// cloning the repository first.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the build command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
fn build_without_clone() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    cmd.arg("build");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: LLVM building cmake failed",
        ))
        .stderr(predicate::str::is_match("The source directory.*does not exist").unwrap());
    Ok(())
}

/// Tests the clone, build, and clean process of the LLVM repository.
///
/// This test verifies that the LLVM repository can be successfully cloned, built, and cleaned.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the build or clean commands.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
#[timeout(std::time::Duration::from_secs(1200))]
fn clone_build_and_clean() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    file.write_str(&*format!("url = \"{}\"", constants::ERA_LLVM_REPO_URL))?;
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(".*Updating files:.*100%.*done").unwrap());
    let mut build_cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    build_cmd.current_dir(path);
    build_cmd
        .arg("build")
        .assert()
        .success()
        .stdout(predicate::str::is_match("Installing:.*").unwrap());
    let mut clean_cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    clean_cmd.current_dir(path);
    clean_cmd.arg("clean");
    clean_cmd.assert().success();
    Ok(())
}

/// Tests the debug build process of the LLVM repository with tests and coverage enabled.
///
/// This test verifies that the LLVM repository can be successfully cloned and built in debug mode
/// with tests and coverage enabled.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the build commands.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
#[timeout(std::time::Duration::from_secs(1200))]
fn debug_build_with_tests_coverage() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    file.write_str(&*format!("url = \"{}\"", constants::ERA_LLVM_REPO_URL))?;
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(".*Updating files:.*100%.*done").unwrap());
    let mut build_cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    build_cmd.current_dir(path);
    build_cmd
        .arg("build")
        .arg("--enable-coverage")
        .arg("--enable-tests")
        .arg("--debug");
    build_cmd
        .assert()
        .success()
        .stdout(predicate::str::is_match("Installing:.*").unwrap());
    Ok(())
}
