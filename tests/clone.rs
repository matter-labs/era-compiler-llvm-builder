pub mod common;

use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use rstest::rstest;

/// Tests the cloning process of the LLVM repository using a specific branch and reference.
///
/// This test verifies that the LLVM repository can be successfully cloned using a specific branch
/// and reference.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the clone command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
fn clone() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    let lockfile = common::create_test_tmp_lockfile(common::ERA_LLVM_REPO_TEST_REF)?;
    let test_dir = lockfile
        .parent()
        .expect("Lockfile parent dir does not exist");
    cmd.current_dir(test_dir);
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "HEAD is now at {}",
            common::ERA_LLVM_REPO_TEST_REF
        )));
    Ok(())
}

/// Tests the shallow cloning process of the LLVM repository using a specific branch and reference.
///
/// This test verifies that the LLVM repository can be successfully cloned using a specific branch
/// and reference with --shallow option.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the clone command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
fn clone_shallow() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    let lockfile = common::create_test_tmp_lockfile(common::ERA_LLVM_REPO_TEST_REF)?;
    let test_dir = lockfile
        .parent()
        .expect("Lockfile parent dir does not exist");
    cmd.current_dir(test_dir);
    cmd.arg("clone");
    cmd.arg("--shallow");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "HEAD is now at {}",
            common::ERA_LLVM_REPO_TEST_REF
        )));
    Ok(())
}

/// Tests the cloning process of the LLVM repository using an invalid reference.
///
/// This test verifies that attempting to clone the LLVM repository using an invalid reference
/// results in a failure.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the clone command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
fn clone_wrong_reference() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    let lockfile = common::create_test_tmp_lockfile(common::ERA_LLVM_REPO_TEST_SHA_INVALID)?;
    let test_dir = lockfile
        .parent()
        .expect("Lockfile parent dir does not exist");
    cmd.current_dir(test_dir);
    cmd.arg("clone");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: LLVM repository commit checking out failed",
    ));
    Ok(())
}

/// Tests the cloning process of the LLVM repository without a lock file.
///
/// This test verifies that attempting to clone the LLVM repository without a lock file
/// results in a failure.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the clone command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
fn clone_without_lockfile() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(common::LLVM_LOCK_FILE)?;
    let path = file.parent().expect("Lockfile parent dir does not exist");
    cmd.current_dir(path);
    cmd.arg("clone");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Error: Error opening \"{}\" file",
            common::LLVM_LOCK_FILE
        )));
    Ok(())
}
