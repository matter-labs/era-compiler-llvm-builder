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
    "-DLLVM_INCLUDE_TESTS=\'Maybe\'"
    ];

pub const SHARED_BUILD_OPTS_NOT_MUSL: [&'static str; 1] = [
    "-DLLVM_TARGETS_TO_BUILD=\'SyncVM\'",
    ];
