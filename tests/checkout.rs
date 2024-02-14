use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;

mod constants;

/// Tests the checkout process after cloning the LLVM repository.
///
/// This test verifies that after cloning the LLVM repository, checking out a specific branch
/// or reference works as expected.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the checkout command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
fn checkout_after_clone() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let lockfile = constants::create_test_tmp_lockfile(constants::ERA_LLVM_REPO_TEST_REF)?;
    let test_dir = lockfile
        .parent()
        .expect("Lockfile parent dir does not exist");
    cmd.current_dir(test_dir);
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(".*Updating files:.*100%.*done").unwrap());
    let mut checkout_cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    checkout_cmd.current_dir(test_dir);
    checkout_cmd.arg("checkout");
    checkout_cmd
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "HEAD is now at {}",
            constants::ERA_LLVM_REPO_TEST_REF
        )));
    Ok(())
}

/// Tests the force checkout process after cloning the LLVM repository.
///
/// This test verifies that after cloning the LLVM repository, checking out a specific branch
/// or reference with the `--force` option works as expected.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the checkout command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
fn force_checkout() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let lockfile = constants::create_test_tmp_lockfile(constants::ERA_LLVM_REPO_TEST_REF)?;
    let test_dir = lockfile
        .parent()
        .expect("Lockfile parent dir does not exist");
    cmd.current_dir(test_dir);
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(".*Updating files:.*100%.*done").unwrap());
    let mut checkout_cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    checkout_cmd.current_dir(test_dir);
    checkout_cmd.arg("checkout").arg("--force");
    checkout_cmd
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "HEAD is now at {}",
            constants::ERA_LLVM_REPO_TEST_REF
        )));
    Ok(())
}

/// Tests the checkout process without a lock file in the LLVM repository.
///
/// This test verifies that attempting to checkout the LLVM repository without a lock file
/// results in a failure.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the checkout command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
fn checkout_without_lockfile() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().expect("Lockfile parent dir does not exist");
    cmd.current_dir(path);
    cmd.arg("checkout");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Error: Error opening \"{}\" file",
            constants::LLVM_LOCK_FILE
        )));
    Ok(())
}
