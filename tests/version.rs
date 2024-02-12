use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
mod constants;
use rstest::rstest;

#[rstest]
#[case("")]
#[case("build")]
#[case("clean")]
#[case("clone")]
#[case("checkout")]
fn version(#[case] subcommand: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    if subcommand != "" {
        cmd.arg(subcommand);
    }
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(constants::PACKAGE_VERSION));
    Ok(())
}
