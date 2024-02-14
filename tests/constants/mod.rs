use assert_fs::fixture::FileWriteStr;

pub const ZKEVM_LLVM: &str = "zkevm-llvm";
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ERA_LLVM_REPO_URL: &str = "https://github.com/matter-labs/era-compiler-llvm";
pub const ERA_LLVM_REPO_TEST_BRANCH: &str = "v1.3.10";
pub const ERA_LLVM_REPO_TEST_REF: &str = "11de9aa24";
pub const ERA_LLVM_REPO_TEST_SHA_INVALID: &str = "12345abcd";
pub const LLVM_LOCK_FILE: &str = "LLVM.lock";

/// Creates a temporary lock file for testing.
pub fn create_test_tmp_lockfile(reference: &str) -> anyhow::Result<assert_fs::NamedTempFile> {
    let file = assert_fs::NamedTempFile::new(LLVM_LOCK_FILE)?;
    let lock = compiler_llvm_builder::Lock {
        url: ERA_LLVM_REPO_URL.to_string(),
        branch: ERA_LLVM_REPO_TEST_BRANCH.to_string(),
        r#ref: Some(reference.to_string()),
    };
    file.write_str(toml::to_string(&lock)?.as_str())?;
    Ok(file)
}
