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
pub fn build(build_type: BuildType, enable_tests: bool) -> anyhow::Result<()> {
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
            format!(
                "-DCMAKE_INSTALL_PREFIX='{}'",
                llvm_target_final.to_string_lossy().as_ref(),
            )
            .as_str(),
            format!("-DCMAKE_BUILD_TYPE='{build_type}'").as_str(),
            "-DCMAKE_C_COMPILER='clang'",
            "-DCMAKE_CXX_COMPILER='clang++'",
            "-DLLVM_TARGETS_TO_BUILD='SyncVM'",
            "-DLLVM_USE_LINKER='lld'",
            format!(
                "-DLLVM_BUILD_UTILS='{}'",
                if enable_tests { "On" } else { "Off" },
            )
            .as_str(),
            format!(
                "-DLLVM_BUILD_TESTS='{}'",
                if enable_tests { "On" } else { "Off" },
            )
            .as_str(),
            format!(
                "-DLLVM_INCLUDE_UTILS='{}'",
                if enable_tests { "On" } else { "Off" },
            )
            .as_str(),
            format!(
                "-DLLVM_INCLUDE_TESTS='{}'",
                if enable_tests { "On" } else { "Off" },
            )
            .as_str(),
        ])
        .args(crate::platforms::SHARED_BUILD_OPTS)
        .args(crate::platforms::SHARED_BUILD_OPTS_NOT_MUSL),
        "LLVM building cmake",
    )?;

    crate::utils::command(
        Command::new("ninja").args(["-C", llvm_build_final.to_string_lossy().as_ref(), "install"]),
        "LLVM building with ninja",
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
