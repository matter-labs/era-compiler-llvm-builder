//!
//! The zkEVM LLVM `macos` builder.
//!

use std::process::Command;

use crate::build_type::BuildType;
use crate::llvm_path::LLVMPath;

///
/// The building sequence.
///
pub fn build(
    build_type: BuildType,
    enable_tests: bool,
    enable_coverage: bool,
    extra_args: Vec<String>,
) -> anyhow::Result<()> {
    crate::utils::check_presence("cmake")?;
    crate::utils::check_presence("ninja")?;

    let llvm_module_llvm = LLVMPath::llvm_module_llvm()?;
    let llvm_build_final = LLVMPath::llvm_build_final()?;
    let llvm_target_final = LLVMPath::llvm_target_final()?;

    crate::utils::command(
        Command::new("cmake")
            .args([
                "-S",
                llvm_module_llvm.to_string_lossy().as_ref(),
                "-B",
                llvm_build_final.to_string_lossy().as_ref(),
                "-G",
                "Ninja",
                format!(
                    "-DCMAKE_INSTALL_PREFIX='{}'",
                    llvm_target_final.to_string_lossy().as_ref(),
                )
                .as_str(),
                format!("-DCMAKE_BUILD_TYPE='{build_type}'").as_str(),
                "-DCMAKE_OSX_DEPLOYMENT_TARGET='11.0'",
            ])
            .args(crate::platforms::shared_build_opts_tests(enable_tests))
            .args(crate::platforms::shared_build_opts_coverage(
                enable_coverage,
            ))
            .args(crate::platforms::SHARED_BUILD_OPTS)
            .args(crate::platforms::SHARED_BUILD_OPTS_NOT_MUSL)
            .args(extra_args),
        "LLVM building cmake",
    )?;

    crate::utils::command(
        Command::new("ninja").args(["-C", llvm_build_final.to_string_lossy().as_ref(), "install"]),
        "LLVM building with ninja",
    )?;

    Ok(())
}
