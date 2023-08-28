//!
//! The zkEVM LLVM builder platforms.
//!

pub mod aarch64_macos;
pub mod x86_64_linux_gnu;
pub mod x86_64_linux_musl;
pub mod x86_64_macos;
pub mod x86_64_windows_gnu;

/// The build options shared by all platforms.
pub const SHARED_BUILD_OPTS: [&str; 18] = [
    "-DPACKAGE_VENDOR='Matter Labs'",
    "-DCLANG_VENDOR='Matter Labs'",
    "-DCLANG_REPOSITORY_STRING='origin'",
    "-DCMAKE_COLOR_DIAGNOSTICS='Off'",
    "-DCMAKE_CXX_FLAGS='-Werror'",
    "-DLLVM_BUILD_DOCS='Off'",
    "-DLLVM_INCLUDE_DOCS='Off'",
    "-DLLVM_INCLUDE_BENCHMARKS='Off'",
    "-DLLVM_INCLUDE_EXAMPLES='Off'",
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
];

/// The build options shared by all platforms except MUSL.
pub const SHARED_BUILD_OPTS_NOT_MUSL: [&str; 7] = [
    "-DLLVM_TARGETS_TO_BUILD=\'SyncVM\'",
    "-DLLVM_DEFAULT_TARGET_TRIPLE='syncvm'",
    "-DLLVM_OPTIMIZED_TABLEGEN='On'",
    "-DLLVM_BUILD_RUNTIME='Off'",
    "-DLLVM_BUILD_RUNTIMES='Off'",
    "-DLLVM_INCLUDE_RUNTIMES='Off'",
    "-DLLVM_ENABLE_ASSERTIONS='On'",
];

/// The LLVM tests build options shared by all platforms.
pub fn shared_build_opts_tests(enabled: bool) -> Vec<String> {
    vec![
        format!(
            "-DLLVM_BUILD_UTILS='{}'",
            if enabled { "On" } else { "Off" },
        ),
        format!(
            "-DLLVM_BUILD_TESTS='{}'",
            if enabled { "On" } else { "Off" },
        ),
        format!(
            "-DLLVM_INCLUDE_UTILS='{}'",
            if enabled { "On" } else { "Off" },
        ),
        format!(
            "-DLLVM_INCLUDE_TESTS='{}'",
            if enabled { "On" } else { "Off" },
        ),
    ]
}

/// The code coverage build options shared by all platforms.
pub fn shared_build_opts_coverage(enabled: bool) -> Vec<String> {
    vec![format!(
        "-DLLVM_BUILD_INSTRUMENTED_COVERAGE='{}'",
        if enabled { "On" } else { "Off" },
    )]
}
