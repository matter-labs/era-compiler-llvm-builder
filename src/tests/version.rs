use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use rstest::rstest;

/// Tests the version command for various subcommands.
///
/// This test verifies that running the version command for different subcommands returns the
/// expected version information.
///
/// # Parameters
///
/// - `subcommand`: The subcommand for which the version is being tested.
///
/// # Errors
///
/// Returns an error if any of the test assertions fail or if there is an error while executing
/// the command.
///
/// # Returns
///
/// Returns `Ok(())` if the test passes.
#[rstest]
#[case("")]
#[case("build")]
#[case("clean")]
#[case("clone")]
#[case("checkout")]
fn version(#[case] subcommand: String) -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(super::common::ZKEVM_LLVM)?;
    if subcommand != "" {
        cmd.arg(subcommand);
    }
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(super::common::PACKAGE_VERSION));
    Ok(())
}
