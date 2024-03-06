pub mod common;

use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use rstest::rstest;

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
    let mut cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(common::LLVM_LOCK_FILE)?;
    let path = file.parent().expect("Lockfile parent dir does not exist");
    cmd.current_dir(path);
    cmd.arg("build");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "building cmake failed",
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
#[timeout(std::time::Duration::from_secs(5000))]
fn clone_build_and_clean() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    let lockfile = common::create_test_tmp_lockfile(common::ERA_LLVM_REPO_TEST_REF)?;
    let test_dir = lockfile
        .parent()
        .expect("Lockfile parent dir does not exist");
    cmd.current_dir(test_dir);
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(".*Updating files:.*100%.*done").unwrap());
    let mut build_cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    build_cmd.current_dir(test_dir);
    build_cmd
        .arg("build")
        .assert()
        .success()
        .stdout(predicate::str::is_match("Installing:.*").unwrap());
    let mut clean_cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    clean_cmd.current_dir(test_dir);
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
#[timeout(std::time::Duration::from_secs(5000))]
#[ignore] // Unsupported for MUSL targets, use --run-ignored to execute for other targets
fn debug_build_with_tests_coverage() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    let lockfile = common::create_test_tmp_lockfile(common::ERA_LLVM_REPO_TEST_REF)?;
    let test_dir = lockfile
        .parent()
        .expect("Lockfile parent dir does not exist");
    cmd.current_dir(test_dir);
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(".*Updating files:.*100%.*done").unwrap());
    let mut build_cmd = Command::cargo_bin(common::ZKEVM_LLVM)?;
    build_cmd.current_dir(test_dir);
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
