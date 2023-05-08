//!
//! The zkEVM LLVM builder platforms.
//!

pub mod aarch64_macos;
pub mod x86_64_linux_gnu;
pub mod x86_64_linux_musl;
pub mod x86_64_macos;
pub mod x86_64_windows_gnu;

pub const SHARED_BUILD_OPTS: [&'static str; 4] = [
    "-DPACKAGE_VENDOR='Matter Labs'",
    "-DCLANG_VENDOR='Matter Labs'",
    "-DCLANG_REPOSITORY_STRING='origin'",
    "-DCMAKE_COLOR_DIAGNOSTICS='Off'",
    "-DLLVM_BUILD_DOCS='Off'",
    "-DLLVM_INCLUDE_DOCS='Off'",
    "-DLLVM_INCLUDE_BENCHMARKS='Off'",
    "-DLLVM_INCLUDE_EXAMPLES='Off'",
    ];

pub const SHARED_BUILD_OPTS_NOT_MUSL: [&'static str; 1] = [
    "-DLLVM_TARGETS_TO_BUILD=\'SyncVM\'",
    "-DLLVM_DEFAULT_TARGET_TRIPLE='syncvm'",
    "-DLLVM_OPTIMIZED_TABLEGEN='On'",
    "-DLLVM_BUILD_RUNTIME='Off'",
    "-DLLVM_BUILD_RUNTIMES='Off'",
    ];
