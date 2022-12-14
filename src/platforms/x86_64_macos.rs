//!
//! The zkEVM LLVM `macos` builder.
//!

use std::process::Command;

use crate::build_type::BuildType;
use crate::llvm_path::LLVMPath;

///
/// The building sequence.
///
pub fn build(build_type: BuildType) -> anyhow::Result<()> {
    crate::utils::check_presence("cmake")?;
    crate::utils::check_presence("ninja")?;

    let llvm_module_llvm = LLVMPath::llvm_module_llvm()?;
    let llvm_build_final = LLVMPath::llvm_build_final()?;
    let llvm_target_final = LLVMPath::llvm_target_final()?;

    crate::utils::command(
        Command::new("cmake").args([
            "-S",
            llvm_module_llvm.to_string_lossy().as_ref(),
            "-B",
            llvm_build_final.to_string_lossy().as_ref(),
            "-G",
            "Ninja",
            "-DPACKAGE_VENDOR='Matter Labs'",
            "-DCLANG_VENDOR='Matter Labs'",
            "-DCLANG_REPOSITORY_STRING='origin'",
            format!(
                "-DCMAKE_INSTALL_PREFIX='{}'",
                llvm_target_final.to_string_lossy().as_ref(),
            )
            .as_str(),
            format!("-DCMAKE_BUILD_TYPE='{}'", build_type).as_str(),
            "-DCMAKE_OSX_DEPLOYMENT_TARGET='11.0'",
            "-DLLVM_TARGETS_TO_BUILD='SyncVM'",
            "-DLLVM_OPTIMIZED_TABLEGEN='On'",
            "-DLLVM_BUILD_TESTS='Off'",
            "-DLLVM_BUILD_DOCS='Off'",
            "-DLLVM_BUILD_RUNTIME='Off'",
            "-DLLVM_BUILD_RUNTIMES='Off'",
            "-DLLVM_BUILD_UTILS='Off'",
            "-DLLVM_INCLUDE_TESTS='Off'",
            "-DLLVM_INCLUDE_DOCS='Off'",
            "-DLLVM_INCLUDE_BENCHMARKS='Off'",
            "-DLLVM_INCLUDE_EXAMPLES='Off'",
            "-DLLVM_INCLUDE_RUNTIMES='Off'",
            "-DLLVM_INCLUDE_UTILS='Off'",
            "-DLLVM_ENABLE_RUNTIMES='Off'",
            "-DLLVM_ENABLE_ASSERTIONS='Off'",
            "-DLLVM_ENABLE_TERMINFO='Off'",
            "-DLLVM_ENABLE_DOXYGEN='Off'",
            "-DLLVM_ENABLE_SPHINX='Off'",
            "-DLLVM_ENABLE_OCAMLDOC='Off'",
            "-DLLVM_ENABLE_ZLIB='Off'",
            "-DLLVM_ENABLE_ZSTD='Off'",
            "-DLLVM_ENABLE_LIBXML2='Off'",
            "-DLLVM_ENABLE_BINDINGS='Off'",
            "-DLLVM_ENABLE_TERMINFO='Off'",
            "-DLLVM_ENABLE_LIBEDIT='Off'",
            "-DLLVM_ENABLE_LIBPFM='Off'",
            "-DLLVM_ENABLE_PIC='Off'",
        ]),
        "LLVM building cmake",
    )?;

    crate::utils::command(
        Command::new("ninja").args(["-C", llvm_build_final.to_string_lossy().as_ref(), "install"]),
        "LLVM building ninja",
    )?;

    Ok(())
}
