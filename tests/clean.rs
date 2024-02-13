use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;

mod constants;

/// Tests the clean process without cloning the LLVM repository.
///
/// This test verifies that attempting to clean the LLVM directory without first cloning it
/// results in a failure.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the clean command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
fn clean_without_clone() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    cmd.arg("clean");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Unable to remove target LLVM directory",
    ));
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
fn clone_build_and_clean() -> anyhow::Result<()> {
    let mut clone_cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    clone_cmd.current_dir(path);
    file.write_str(&*format!("url = \"{}\"", constants::ERA_LLVM_REPO_URL))?;
    clone_cmd.arg("clone");
    clone_cmd
        .assert()
        .success()
        .stderr(predicate::str::is_match(".*Updating files:.*100%.*done").unwrap());
    let mut build_cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    build_cmd.current_dir(path);
    build_cmd.arg("build").env("DRY_RUN", "true");
    build_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Install the project..."));
    let mut clean_cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    clean_cmd.current_dir(path);
    clean_cmd.arg("clean");
    clean_cmd.assert().success();
    Ok(())
}
