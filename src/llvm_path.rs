//!
//! The ZKsync LLVM builder constants.
//!

use std::path::PathBuf;

///
/// The LLVM path resolver.
///
pub struct LLVMPath {}

impl LLVMPath {
    /// The LLVM host source directory for stage 1 of multistage MUSL builds.
    pub const DIRECTORY_LLVM_HOST_SOURCE: &'static str = "./llvm-host/";

    /// The LLVM source directory.
    pub const DIRECTORY_LLVM_SOURCE: &'static str = "./llvm/";

    /// The LLVM target directory.
    pub const DIRECTORY_LLVM_TARGET: &'static str = "./target-llvm/";

    ///
    /// Returns the path to the `llvm` stage 1 host LLVM source module directory.
    ///
    pub fn llvm_host_module_llvm() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_HOST_SOURCE);
        path.push("llvm");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the `llvm` LLVM source module directory.
    ///
    pub fn llvm_module_llvm() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_SOURCE);
        path.push("llvm");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the MUSL source.
    ///
    pub fn musl_source(name: &str) -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push(name);
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the MUSL build directory.
    ///
    pub fn musl_build(source_directory: &str) -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push(source_directory);
        path.push("build");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the LLVM CRT build directory.
    ///
    pub fn llvm_build_crt() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push("build-crt");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the LLVM host build directory.
    ///
    pub fn llvm_build_host() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push("build-host");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the LLVM final build directory.
    ///
    pub fn llvm_build_final() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push("build-final");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the MUSL target directory.
    ///
    pub fn musl_target() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push("target-musl");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the LLVM CRT target directory.
    ///
    pub fn llvm_target_crt() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push("target-crt");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the LLVM host target directory.
    ///
    pub fn llvm_target_host() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push("target-host");
        crate::utils::absolute_path(path)
    }

    ///
    /// Returns the path to the LLVM final target directory.
    ///
    pub fn llvm_target_final() -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::from(Self::DIRECTORY_LLVM_TARGET);
        path.push("target-final");
        crate::utils::absolute_path(path)
    }
}
