use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;

mod constants;

/// Tests the cloning process of the LLVM repository using the default branch.
///
/// This test verifies that the LLVM repository can be successfully cloned using the default branch.
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
fn clone_default_branch() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    file.write_str(&*format!("url = \"{}\"", constants::ERA_LLVM_REPO_URL))?;
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(".*Updating files:.*100%.*done").unwrap());
    Ok(())
}

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
fn clone_branch_and_ref() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    file.write_str(&*format!(
        "url = \"{}\"\nbranch = \"{}\"\nref = \"{}\"",
        constants::ERA_LLVM_REPO_URL,
        constants::ERA_LLVM_REPO_TEST_BRANCH,
        constants::ERA_LLVM_REPO_TEST_REF
    ))?;
    cmd.arg("clone");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "HEAD is now at {}",
            constants::ERA_LLVM_REPO_TEST_REF
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
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    file.write_str(&*format!(
        "url = \"{}\"\nbranch = \"{}\"\nref = \"{}\"",
        constants::ERA_LLVM_REPO_URL,
        constants::ERA_LLVM_REPO_TEST_BRANCH,
        constants::ERA_LLVM_REPO_TEST_SHA_INVALID
    ))?;
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
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    cmd.arg("clone");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
