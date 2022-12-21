//!
//! The zkEVM LLVM `windows-gnu` builder.
//!

use std::path::PathBuf;
use std::process::Command;

use crate::build_type::BuildType;
use crate::llvm_path::LLVMPath;

///
/// The building sequence.
///
pub fn build(build_type: BuildType) -> anyhow::Result<()> {
    crate::utils::check_presence("cmake")?;
    crate::utils::check_presence("clang")?;
    crate::utils::check_presence("clang++")?;
    crate::utils::check_presence("lld")?;
    crate::utils::check_presence("ninja")?;

    let llvm_module_llvm =
        LLVMPath::llvm_module_llvm().and_then(crate::utils::path_windows_to_unix)?;
    let llvm_build_final =
        LLVMPath::llvm_build_final().and_then(crate::utils::path_windows_to_unix)?;
    let llvm_target_final =
        LLVMPath::llvm_target_final().and_then(crate::utils::path_windows_to_unix)?;

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
            "-DCMAKE_C_COMPILER='clang'",
            "-DCMAKE_CXX_COMPILER='clang++'",
            "-DCMAKE_COLOR_DIAGNOSTICS='Off'",
            "-DLLVM_TARGETS_TO_BUILD='SyncVM'",
            "-DLLVM_OPTIMIZED_TABLEGEN='On'",
            "-DLLVM_USE_LINKER='lld'",
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
            "-DLLVM_ENABLE_ASSERTIONS='On'",
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
        ]),
        "LLVM building cmake",
    )?;

    crate::utils::command(
        Command::new("ninja").args(["-C", llvm_build_final.to_string_lossy().as_ref(), "install"]),
        "LLVM building ninja",
    )?;

    let libstdcpp_source_path = match std::env::var("LIBSTDCPP_SOURCE_PATH") {
        Ok(libstdcpp_source_path) => PathBuf::from(libstdcpp_source_path),
        Err(error) => anyhow::bail!(
            "The `LIBSTDCPP_SOURCE_PATH` must be set to the path to the libstdc++.a static library: {}", error
        ),
    };
    let mut libstdcpp_destination_path = llvm_target_final;
    libstdcpp_destination_path.push("./lib/libstdc++.a");
    fs_extra::file::copy(
        crate::utils::path_windows_to_unix(libstdcpp_source_path)?,
        crate::utils::path_windows_to_unix(libstdcpp_destination_path)?,
        &fs_extra::file::CopyOptions::default(),
    )?;

    Ok(())
}
