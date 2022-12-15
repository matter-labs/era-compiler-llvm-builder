//!
//! The zkEVM LLVM `linux-musl` builder.
//!

use std::path::Path;
use std::process::Command;

use crate::build_type::BuildType;
use crate::llvm_path::LLVMPath;

///
/// The building sequence.
///
pub fn build(build_type: BuildType) -> anyhow::Result<()> {
    crate::utils::check_presence("wget")?;
    crate::utils::check_presence("tar")?;
    crate::utils::check_presence("cmake")?;
    crate::utils::check_presence("clang")?;
    crate::utils::check_presence("clang++")?;
    crate::utils::check_presence("lld")?;
    crate::utils::check_presence("ninja")?;

    let musl_name = "musl-1.2.3";
    let musl_build = LLVMPath::musl_build(musl_name)?;
    let musl_target = LLVMPath::musl_target()?;

    let llvm_module_llvm = LLVMPath::llvm_module_llvm()?;

    let llvm_build_crt = LLVMPath::llvm_build_crt()?;
    let llvm_target_crt = LLVMPath::llvm_target_crt()?;

    let llvm_build_host = LLVMPath::llvm_build_host()?;
    let llvm_target_host = LLVMPath::llvm_target_host()?;

    let llvm_build_final = LLVMPath::llvm_build_final()?;
    let llvm_target_final = LLVMPath::llvm_target_final()?;

    download_musl(musl_name)?;
    build_musl(musl_build.as_path(), musl_target.as_path())?;
    build_crt(
        llvm_module_llvm.as_path(),
        llvm_build_crt.as_path(),
        llvm_target_crt.as_path(),
    )?;
    build_host(
        llvm_module_llvm.as_path(),
        llvm_build_host.as_path(),
        llvm_target_host.as_path(),
        musl_target.as_path(),
        llvm_target_crt.as_path(),
    )?;
    build_target(
        build_type,
        llvm_module_llvm.as_path(),
        llvm_build_final.as_path(),
        llvm_target_final.as_path(),
        musl_target.as_path(),
        llvm_target_host.as_path(),
    )?;

    Ok(())
}

///
/// The `musl` downloading sequence.
///
fn download_musl(name: &str) -> anyhow::Result<()> {
    let tar_file_name = format!("{}.tar.gz", name);
    let url = format!(
        "https://git.musl-libc.org/cgit/musl/snapshot/{}",
        tar_file_name
    );

    crate::utils::command(
        Command::new("wget")
            .current_dir(LLVMPath::DIRECTORY_LLVM_TARGET)
            .arg("--verbose")
            .arg("--output-document")
            .arg(tar_file_name.as_str())
            .arg(url),
        "MUSL downloading",
    )?;

    crate::utils::command(
        Command::new("tar")
            .current_dir(LLVMPath::DIRECTORY_LLVM_TARGET)
            .arg("-x")
            .arg("-v")
            .arg("-z")
            .arg("-f")
            .arg(tar_file_name.as_str()),
        "MUSL unpacking",
    )?;

    Ok(())
}

///
/// The `musl` building sequence.
///
fn build_musl(build_directory: &Path, target_directory: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(build_directory)?;
    std::fs::create_dir_all(target_directory)?;

    crate::utils::command(
        Command::new("../configure")
            .current_dir(build_directory)
            .arg(format!("--prefix={}", target_directory.to_string_lossy()))
            .arg(format!(
                "--syslibdir={}/lib/",
                target_directory.to_string_lossy()
            ))
            .arg("--enable-wrapper='clang'"),
        "MUSL configuring",
    )?;
    crate::utils::command(
        Command::new("make")
            .current_dir(build_directory)
            .arg("-j")
            .arg(num_cpus::get().to_string()),
        "MUSL building",
    )?;
    crate::utils::command(
        Command::new("make")
            .current_dir(build_directory)
            .arg("install"),
        "MUSL installing",
    )?;

    let mut include_directory = target_directory.to_path_buf();
    include_directory.push("include/");

    let mut asm_include_directory = include_directory.clone();
    asm_include_directory.push("asm/");
    std::fs::create_dir_all(asm_include_directory.as_path())?;

    let mut types_header_path = asm_include_directory.clone();
    types_header_path.push("types.h");

    let copy_options = fs_extra::dir::CopyOptions {
        overwrite: true,
        copy_inside: true,
        ..Default::default()
    };
    fs_extra::dir::copy("/usr/include/linux", include_directory, &copy_options)?;

    let copy_options = fs_extra::dir::CopyOptions {
        overwrite: true,
        copy_inside: true,
        content_only: true,
        ..Default::default()
    };
    fs_extra::dir::copy(
        "/usr/include/asm-generic",
        asm_include_directory,
        &copy_options,
    )?;

    crate::utils::command(
        Command::new("sed")
            .arg("-i")
            .arg("s/asm-generic/asm/")
            .arg(types_header_path),
        "types_header asm signature replacement",
    )?;

    Ok(())
}

///
/// The `crt` building sequence.
///
fn build_crt(
    source_directory: &Path,
    build_directory: &Path,
    target_directory: &Path,
) -> anyhow::Result<()> {
    crate::utils::command(
        Command::new("cmake").args([
            "-S",
            source_directory.to_string_lossy().as_ref(),
            "-B",
            build_directory.to_string_lossy().as_ref(),
            "-G",
            "Ninja",
            "-DPACKAGE_VENDOR='Matter Labs'",
            "-DCLANG_VENDOR='Matter Labs'",
            "-DCLANG_REPOSITORY_STRING='origin'",
            format!(
                "-DCMAKE_INSTALL_PREFIX='{}'",
                target_directory.to_string_lossy()
            )
            .as_str(),
            "-DCMAKE_BUILD_TYPE='Release'",
            "-DCMAKE_C_COMPILER='clang'",
            "-DCMAKE_CXX_COMPILER='clang++'",
            "-DCMAKE_COLOR_DIAGNOSTICS='Off'",
            "-DLLVM_ENABLE_PROJECTS='compiler-rt'",
            "-DLLVM_TARGETS_TO_BUILD='X86'",
            "-DLLVM_DEFAULT_TARGET_TRIPLE='x86_64-pc-linux-musl'",
            "-DLLVM_BUILD_TESTS='Off'",
            "-DLLVM_BUILD_DOCS='Off'",
            "-DLLVM_BUILD_RUNTIMES='Off'",
            "-DLLVM_BUILD_UTILS='Off'",
            "-DLLVM_INCLUDE_TESTS='Off'",
            "-DLLVM_INCLUDE_DOCS='Off'",
            "-DLLVM_INCLUDE_BENCHMARKS='Off'",
            "-DLLVM_INCLUDE_EXAMPLES='Off'",
            "-DLLVM_INCLUDE_RUNTIMES='Off'",
            "-DLLVM_INCLUDE_UTILS='Off'",
            "-DLLVM_ENABLE_ASSERTIONS='Off'",
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
            "-DCOMPILER_RT_DEFAULT_TARGET_ARCH='x86_64'",
            "-DCOMPILER_RT_BUILD_CRT='On'",
            "-DCOMPILER_RT_BUILD_SANITIZERS='Off'",
            "-DCOMPILER_RT_BUILD_XRAY='Off'",
            "-DCOMPILER_RT_BUILD_LIBFUZZER='Off'",
            "-DCOMPILER_RT_BUILD_PROFILE='Off'",
            "-DCOMPILER_RT_BUILD_MEMPROF='Off'",
            "-DCOMPILER_RT_BUILD_ORC='Off'",
        ]),
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
) -> anyhow::Result<()> {
    crate::utils::command(
        Command::new("cmake").args([
            "-S",
            source_directory.to_string_lossy().as_ref(),
            "-B",
            build_directory.to_string_lossy().as_ref(),
            "-G",
            "Ninja",
            "-DPACKAGE_VENDOR='Matter Labs'",
            "-DCLANG_VENDOR='Matter Labs'",
            "-DCLANG_REPOSITORY_STRING='origin'",
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
            "-DCMAKE_COLOR_DIAGNOSTICS='Off'",
            "-DCLANG_DEFAULT_CXX_STDLIB='libc++'",
            "-DCLANG_DEFAULT_RTLIB='compiler-rt'",
            "-DLLVM_DEFAULT_TARGET_TRIPLE='x86_64-pc-linux-musl'",
            "-DLLVM_TARGETS_TO_BUILD='X86'",
            "-DLLVM_BUILD_TESTS='Off'",
            "-DLLVM_BUILD_DOCS='Off'",
            "-DLLVM_BUILD_UTILS='Off'",
            "-DLLVM_INCLUDE_TESTS='Off'",
            "-DLLVM_INCLUDE_DOCS='Off'",
            "-DLLVM_INCLUDE_BENCHMARKS='Off'",
            "-DLLVM_INCLUDE_EXAMPLES='Off'",
            "-DLLVM_INCLUDE_UTILS='Off'",
            "-DLLVM_ENABLE_PROJECTS='clang;lld'",
            "-DLLVM_ENABLE_RUNTIMES='compiler-rt;libcxx;libcxxabi;libunwind'",
            "-DLLVM_ENABLE_ASSERTIONS='Off'",
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
        ]),
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
fn build_target(
    build_type: BuildType,
    source_directory: &Path,
    build_directory: &Path,
    target_directory: &Path,
    musl_target_directory: &Path,
    host_target_directory: &Path,
) -> anyhow::Result<()> {
    let mut clang_path = host_target_directory.to_path_buf();
    clang_path.push("bin/clang");

    let mut clang_cxx_path = host_target_directory.to_path_buf();
    clang_cxx_path.push("bin/clang++");

    crate::utils::command(
        Command::new("cmake").args([
            "-S",
            source_directory.to_string_lossy().as_ref(),
            "-B",
            build_directory.to_string_lossy().as_ref(),
            "-G",
            "Ninja",
            "-DPACKAGE_VENDOR='Matter Labs'",
            "-DCLANG_VENDOR='Matter Labs'",
            "-DCLANG_REPOSITORY_STRING='origin'",
            "-DBUILD_SHARED_LIBS='Off'",
            "-DLINKER_SUPPORTS_COLOR_DIAGNOSTICS=0",
            format!(
                "-DCMAKE_INSTALL_PREFIX='{}'",
                target_directory.to_string_lossy()
            )
            .as_str(),
            format!("-DCMAKE_BUILD_TYPE='{}'", build_type).as_str(),
            format!("-DCMAKE_C_COMPILER='{}'", clang_path.to_string_lossy()).as_str(),
            format!(
                "-DCMAKE_CXX_COMPILER='{}'",
                clang_cxx_path.to_string_lossy()
            )
            .as_str(),
            "-DCMAKE_COLOR_DIAGNOSTICS='Off'",
            "-DCMAKE_FIND_LIBRARY_SUFFIXES='.a'",
            "-DCMAKE_EXE_LINKER_FLAGS='-fuse-ld=lld -static'",
            "-DLLVM_DEFAULT_TARGET_TRIPLE='x86_64-pc-linux-musl'",
            "-DLLVM_TARGETS_TO_BUILD='SyncVM'",
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
            "-DLLVM_ENABLE_PROJECTS='llvm'",
            "-DLLVM_ENABLE_ASSERTIONS='Off'",
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
        "LLVM target building cmake",
    )?;

    crate::utils::command(
        Command::new("ninja")
            .arg("-C")
            .arg(build_directory)
            .arg("install"),
        "LLVM target building ninja",
    )?;

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
