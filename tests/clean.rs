use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;

mod constants;

#[rstest]
fn clean_without_clone() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    let file = assert_fs::NamedTempFile::new(constants::LLVM_LOCK_FILE)?;
    let path = file.parent().unwrap();
    cmd.current_dir(path);
    cmd.arg("clean");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: unable to remove LLVM directory",
    ));
    Ok(())
}

#[rstest]
fn clone_build_and_clean() -> Result<(), Box<dyn std::error::Error>> {
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
