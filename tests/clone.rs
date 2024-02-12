use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;

mod constants;

#[rstest]
fn clone_default_branch() -> Result<(), Box<dyn std::error::Error>> {
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

#[rstest]
fn clone_branch_and_ref() -> Result<(), Box<dyn std::error::Error>> {
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

#[rstest]
fn clone_wrong_reference() -> Result<(), Box<dyn std::error::Error>> {
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

#[rstest]
fn clone_without_lockfile() -> Result<(), Box<dyn std::error::Error>> {
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
