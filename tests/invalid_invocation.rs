use assert_cmd::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;
mod constants;

#[rstest]
#[case("", "--invalid-option")]
#[case("build", "--invalid-build-option")]
#[case("clean", "--invalid-clean-option")]
#[case("clone", "--invalid-clone-option")]
#[case("checkout", "--invalid-checkout-option")]
fn invalid_option(
    #[case] subcommand: &str,
    #[case] option: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    if subcommand != "" {
        cmd.arg(subcommand);
    }
    cmd.arg(option);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "error: Found argument \'{}\' which wasn't expected",
            option
        )));
    Ok(())
}

#[rstest]
#[case("invalid-subcommand")]
#[case("123")]
#[case("$$.@!;-a3")]
fn invalid_subcommand(#[case] subcommand: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(constants::ZKEVM_LLVM)?;
    cmd.arg(subcommand);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "error: Found argument \'{}\' which wasn't expected",
            subcommand
        )));
    Ok(())
}
