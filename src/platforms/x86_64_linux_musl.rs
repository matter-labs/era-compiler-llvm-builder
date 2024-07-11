//!
//! The ZKsync LLVM amd64 `linux-musl` builder.
//!

use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

use crate::build_type::BuildType;
use crate::llvm_path::LLVMPath;
use crate::platforms::Platform;
use crate::sanitizer::Sanitizer;
use crate::target_triple::TargetTriple;

///
/// The building sequence.
///
#[allow(clippy::too_many_arguments)]
pub fn build(
    build_type: BuildType,
    targets: HashSet<Platform>,
    default_target: Option<TargetTriple>,
    enable_tests: bool,
    enable_coverage: bool,
    extra_args: Vec<String>,
    use_ccache: bool,
    enable_assertions: bool,
    sanitizer: Option<Sanitizer>,
) -> anyhow::Result<()> {
    crate::utils::check_presence("cmake")?;
    crate::utils::check_presence("clang")?;
    crate::utils::check_presence("clang++")?;
    crate::utils::check_presence("lld")?;
    crate::utils::check_presence("ninja")?;

    let musl_name = "musl-1.2.3";
    let musl_build = LLVMPath::musl_build(musl_name)?;
    let musl_target = LLVMPath::musl_target()?;

    let llvm_module_llvm = LLVMPath::llvm_module_llvm()?;
    let llvm_host_module_llvm = LLVMPath::llvm_host_module_llvm()?;

    let llvm_build_crt = LLVMPath::llvm_build_crt()?;
    let llvm_target_crt = LLVMPath::llvm_target_crt()?;

    let llvm_build_host = LLVMPath::llvm_build_host()?;
    let llvm_target_host = LLVMPath::llvm_target_host()?;

    let llvm_build_final = LLVMPath::llvm_build_final()?;
    let llvm_target_final = LLVMPath::llvm_target_final()?;

    if !LLVMPath::musl_source(musl_name)?.exists() {
        crate::utils::download_musl(musl_name)?;
    }
    crate::platforms::shared::build_musl(musl_build.as_path(), musl_target.as_path())?;
    build_crt(
        targets.clone(),
        llvm_host_module_llvm.as_path(),
        llvm_build_crt.as_path(),
        llvm_target_crt.as_path(),
        use_ccache,
    )?;
    build_host(
        llvm_host_module_llvm.as_path(),
        llvm_build_host.as_path(),
        llvm_target_host.as_path(),
        musl_target.as_path(),
        llvm_target_crt.as_path(),
        use_ccache,
    )?;
    build_target(
        build_type,
        targets,
        default_target,
        llvm_module_llvm.as_path(),
        llvm_build_final.as_path(),
        llvm_target_final.as_path(),
        musl_target.as_path(),
        llvm_target_host.as_path(),
        enable_tests,
        enable_coverage,
        extra_args,
        use_ccache,
        enable_assertions,
        sanitizer,
    )?;

    Ok(())
}

///
/// The `crt` building sequence.
///
fn build_crt(
    mut targets: HashSet<Platform>,
    source_directory: &Path,
    build_directory: &Path,
    target_directory: &Path,
    use_ccache: bool,
) -> anyhow::Result<()> {
    targets.insert(Platform::X86);

    crate::utils::command(
        Command::new("cmake")
            .args([
                "-S",
                source_directory.to_string_lossy().as_ref(),
                "-B",
                build_directory.to_string_lossy().as_ref(),
                "-G",
                "Ninja",
                format!(
                    "-DCMAKE_INSTALL_PREFIX='{}'",
                    target_directory.to_string_lossy()
                )
                .as_str(),
                "-DCMAKE_BUILD_TYPE='Release'",
                "-DCMAKE_C_COMPILER='clang'",
                "-DCMAKE_CXX_COMPILER='clang++'",
                "-DLLVM_ENABLE_PROJECTS='compiler-rt'",
                format!("-DLLVM_TARGETS_TO_BUILD='{}'", Platform::X86).as_str(),
                "-DLLVM_DEFAULT_TARGET_TRIPLE='x86_64-pc-linux-musl'",
                "-DLLVM_BUILD_TESTS='Off'",
                "-DLLVM_BUILD_RUNTIMES='Off'",
                "-DLLVM_BUILD_UTILS='Off'",
                "-DLLVM_INCLUDE_TESTS='Off'",
                "-DLLVM_INCLUDE_RUNTIMES='Off'",
                "-DLLVM_INCLUDE_UTILS='Off'",
                "-DCOMPILER_RT_DEFAULT_TARGET_ARCH='x86_64'",
                "-DCOMPILER_RT_BUILD_CRT='On'",
                "-DCOMPILER_RT_BUILD_SANITIZERS='Off'",
                "-DCOMPILER_RT_BUILD_XRAY='Off'",
                "-DCOMPILER_RT_BUILD_LIBFUZZER='Off'",
                "-DCOMPILER_RT_BUILD_PROFILE='Off'",
                "-DCOMPILER_RT_BUILD_MEMPROF='Off'",
                "-DCOMPILER_RT_BUILD_ORC='Off'",
            ])
            .args(crate::platforms::shared::SHARED_BUILD_OPTS)
            .args(crate::platforms::shared::shared_build_opts_ccache(
                use_ccache,
            )),
        "CRT building cmake",
    )?;

    crate::utils::command(
        Command::new("ninja")
            .arg("-C")
            .arg(build_directory)
            .arg("install-crt"),
        "CRT building ninja",
    )?;

    Ok(())
}

///
/// The host toolchain building sequence.
///
fn build_host(
    source_directory: &Path,
    build_directory: &Path,
    target_directory: &Path,
    musl_target_directory: &Path,
    crt_target_directory: &Path,
    use_ccache: bool,
) -> anyhow::Result<()> {
    crate::utils::command(
        Command::new("cmake")
            .args([
                "-S",
                source_directory.to_string_lossy().as_ref(),
                "-B",
                build_directory.to_string_lossy().as_ref(),
                "-G",
                "Ninja",
                format!(
                    "-DDEFAULT_SYSROOT='{}'",
                    musl_target_directory.to_string_lossy()
                )
                .as_str(),
                "-DLINKER_SUPPORTS_COLOR_DIAGNOSTICS=0",
                format!(
                    "-DCMAKE_INSTALL_PREFIX='{}'",
                    target_directory.to_string_lossy()
                )
                .as_str(),
                "-DCMAKE_BUILD_TYPE='Release'",
                "-DCMAKE_C_COMPILER='clang'",
                "-DCMAKE_CXX_COMPILER='clang++'",
                "-DCLANG_DEFAULT_CXX_STDLIB='libc++'",
                "-DCLANG_DEFAULT_RTLIB='compiler-rt'",
                "-DLLVM_DEFAULT_TARGET_TRIPLE='x86_64-pc-linux-musl'",
                "-DLLVM_TARGETS_TO_BUILD='X86'",
                "-DLLVM_BUILD_TESTS='Off'",
                "-DLLVM_BUILD_UTILS='Off'",
                "-DLLVM_INCLUDE_TESTS='Off'",
                "-DLLVM_INCLUDE_UTILS='Off'",
                "-DLLVM_ENABLE_PROJECTS='clang;lld'",
                "-DLLVM_ENABLE_RUNTIMES='compiler-rt;libcxx;libcxxabi;libunwind'",
                "-DLIBCXX_CXX_ABI='libcxxabi'",
                "-DLIBCXX_HAS_MUSL_LIBC='On'",
                "-DLIBCXX_ENABLE_SHARED='Off'",
                "-DLIBCXX_ENABLE_STATIC='On'",
                "-DLIBCXX_ENABLE_STATIC_ABI_LIBRARY='On'",
                "-DLIBCXXABI_ENABLE_SHARED='Off'",
                "-DLIBCXXABI_ENABLE_STATIC='On'",
                "-DLIBCXXABI_ENABLE_STATIC_UNWINDER='On'",
                "-DLIBCXXABI_USE_LLVM_UNWINDER='On'",
                "-DLIBCXXABI_USE_COMPILER_RT='On'",
                "-DLIBUNWIND_ENABLE_STATIC='On'",
                "-DLIBUNWIND_ENABLE_SHARED='Off'",
                "-DCOMPILER_RT_BUILD_CRT='On'",
                "-DCOMPILER_RT_BUILD_SANITIZERS='Off'",
                "-DCOMPILER_RT_BUILD_XRAY='Off'",
                "-DCOMPILER_RT_BUILD_LIBFUZZER='Off'",
                "-DCOMPILER_RT_BUILD_PROFILE='Off'",
                "-DCOMPILER_RT_BUILD_MEMPROF='Off'",
                "-DCOMPILER_RT_BUILD_ORC='Off'",
                "-DCOMPILER_RT_DEFAULT_TARGET_ARCH='x86_64'",
                "-DCOMPILER_RT_DEFAULT_TARGET_ONLY='On'",
            ])
            .args(crate::platforms::shared::SHARED_BUILD_OPTS)
            .args(crate::platforms::shared::shared_build_opts_ccache(
                use_ccache,
            )),
        "LLVM host building cmake",
    )?;

    let mut crt_lib_directory = crt_target_directory.to_path_buf();
    crt_lib_directory.push("lib/");

    let mut build_lib_directory = build_directory.to_path_buf();
    build_lib_directory.push("lib/");

    let copy_options = fs_extra::dir::CopyOptions {
        overwrite: true,
        copy_inside: true,
        content_only: true,
        ..Default::default()
    };
    fs_extra::dir::copy(crt_lib_directory, build_lib_directory, &copy_options)?;

    crate::utils::command(
        Command::new("ninja")
            .arg("-C")
            .arg(build_directory)
            .arg("install"),
        "LLVM host building ninja",
    )?;

    Ok(())
}

///
/// The target toolchain building sequence.
///
#[allow(clippy::too_many_arguments)]
fn build_target(
    build_type: BuildType,
    targets: HashSet<Platform>,
    default_target: Option<TargetTriple>,
    source_directory: &Path,
    build_directory: &Path,
    target_directory: &Path,
    musl_target_directory: &Path,
    host_target_directory: &Path,
    enable_tests: bool,
    enable_coverage: bool,
    extra_args: Vec<String>,
    use_ccache: bool,
    enable_assertions: bool,
    sanitizer: Option<Sanitizer>,
) -> anyhow::Result<()> {
    let mut clang_path = host_target_directory.to_path_buf();
    clang_path.push("bin/clang");

    let mut clang_cxx_path = host_target_directory.to_path_buf();
    clang_cxx_path.push("bin/clang++");

    crate::utils::command(
        Command::new("cmake")
            .args([
                "-S",
                source_directory.to_string_lossy().as_ref(),
                "-B",
                build_directory.to_string_lossy().as_ref(),
                "-G",
                "Ninja",
                "-DBUILD_SHARED_LIBS='Off'",
                "-DLINKER_SUPPORTS_COLOR_DIAGNOSTICS=0",
                format!(
                    "-DCMAKE_INSTALL_PREFIX='{}'",
                    target_directory.to_string_lossy()
                )
                .as_str(),
                format!("-DCMAKE_BUILD_TYPE='{build_type}'").as_str(),
                format!("-DCMAKE_C_COMPILER='{}'", clang_path.to_string_lossy()).as_str(),
                format!(
                    "-DCMAKE_CXX_COMPILER='{}'",
                    clang_cxx_path.to_string_lossy()
                )
                .as_str(),
                "-DCMAKE_FIND_LIBRARY_SUFFIXES='.a'",
                "-DCMAKE_BUILD_WITH_INSTALL_RPATH=1",
                "-DCMAKE_EXE_LINKER_FLAGS='-fuse-ld=lld -static'",
                format!(
                    "-DLLVM_TARGETS_TO_BUILD='{}'",
                    targets
                        .into_iter()
                        .map(|platform| platform.to_string())
                        .collect::<Vec<String>>()
                        .join(";")
                )
                .as_str(),
                "-DLLVM_ENABLE_PROJECTS='llvm;lld'",
            ])
            .args(crate::platforms::shared::shared_build_opts_default_target(
                default_target,
            ))
            .args(crate::platforms::shared::SHARED_BUILD_OPTS)
            .args(crate::platforms::shared::SHARED_BUILD_OPTS_NOT_MUSL)
            .args(crate::platforms::shared::shared_build_opts_tests(
                enable_tests,
            ))
            .args(crate::platforms::shared::shared_build_opts_coverage(
                enable_coverage,
            ))
            .args(extra_args)
            .args(crate::platforms::shared::shared_build_opts_ccache(
                use_ccache,
            ))
            .args(crate::platforms::shared::shared_build_opts_assertions(
                enable_assertions,
            ))
            .args(crate::platforms::shared::shared_build_opts_sanitizers(
                sanitizer,
            )),
        "LLVM target building cmake",
    )?;

    crate::utils::ninja(build_directory)?;

    let mut musl_lib_directory = musl_target_directory.to_path_buf();
    musl_lib_directory.push("lib/");

    let mut host_lib_directory = host_target_directory.to_path_buf();
    host_lib_directory.push("lib/x86_64-pc-linux-musl/");

    let mut target_lib_directory = target_directory.to_path_buf();
    target_lib_directory.push("lib/");

    let copy_options = fs_extra::dir::CopyOptions {
        overwrite: true,
        copy_inside: true,
        content_only: true,
        ..Default::default()
    };
    fs_extra::dir::copy(
        musl_lib_directory,
        target_lib_directory.as_path(),
        &copy_options,
    )?;
    fs_extra::dir::copy(
        host_lib_directory,
        target_lib_directory.as_path(),
        &copy_options,
    )?;

    Ok(())
}
