use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;

mod constants;

#[rstest]
fn build_without_clone() -> Result<(), Box<dyn std::error::Error>> {
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

#[rstest]
fn build() -> Result<(), Box<dyn std::error::Error>> {
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
    build_cmd.arg("build").env("DRY_RUN", "true");
    build_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Install the project..."));
    Ok(())
}

#[rstest]
fn debug_build_with_cache_tests_coverage() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg("--use-ccache")
        .arg("--enable-coverage")
        .arg("--enable-tests")
        .arg("--debug")
        .env("DRY_RUN", "true");
    build_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Install the project..."));
    Ok(())
}
