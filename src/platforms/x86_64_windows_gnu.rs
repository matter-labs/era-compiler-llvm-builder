//!
//! The zkEVM LLVM amd64 `windows-gnu` builder.
//!

use std::path::PathBuf;
use std::process::Command;

use crate::build_type::BuildType;
use crate::llvm_path::LLVMPath;
use crate::platforms::Platform;

///
/// The building sequence.
///
pub fn build(
    build_type: BuildType,
    targets: Vec<Platform>,
    enable_tests: bool,
    enable_coverage: bool,
    extra_args: Vec<String>,
    use_ccache: bool,
) -> anyhow::Result<()> {
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
                "-DCMAKE_C_COMPILER='clang'",
                "-DCMAKE_CXX_COMPILER='clang++'",
                format!(
                    "-DLLVM_TARGETS_TO_BUILD='{}'",
                    targets
                        .into_iter()
                        .map(|platform| platform.to_string())
                        .collect::<Vec<String>>()
                        .join(";")
                )
                .as_str(),
                "-DLLVM_USE_LINKER='lld'",
            ])
            .args(crate::platforms::shared::shared_build_opts_tests(
                enable_tests,
            ))
            .args(crate::platforms::shared::shared_build_opts_coverage(
                enable_coverage,
            ))
            .args(crate::platforms::shared::SHARED_BUILD_OPTS)
            .args(crate::platforms::shared::SHARED_BUILD_OPTS_NOT_MUSL)
            .args(extra_args)
            .args(crate::platforms::shared::shared_build_opts_ccache(
                use_ccache,
            )),
        "LLVM building cmake",
    )?;

    crate::utils::ninja(llvm_build_final.as_ref())?;

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
