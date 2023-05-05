//!
//! The zkEVM LLVM builder platforms.
//!

pub mod aarch64_macos;
pub mod x86_64_linux_gnu;
pub mod x86_64_linux_musl;
pub mod x86_64_macos;
pub mod x86_64_windows_gnu;

pub const SHARED_BUILD_OPTS: [&'static str; 3] = ["-DPACKAGE_VENDOR=\'Utter Labs\'", "-DLLVM_TARGETS_TO_BUILD=\'SyncVM\'", "-DLLVM_INCLUDE_TESTS=\'Maybe\'"]; 