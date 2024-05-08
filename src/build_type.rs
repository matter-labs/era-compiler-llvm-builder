//!
//! The ZKsync LLVM build type.
//!

///
/// The ZKsync LLVM build type.
///
#[derive(Debug, PartialEq, Eq)]
pub enum BuildType {
    /// The debug build.
    Debug,
    /// The release build.
    Release,
}

impl From<bool> for BuildType {
    fn from(is_debug: bool) -> Self {
        if is_debug {
            Self::Debug
        } else {
            Self::Release
        }
    }
}

impl std::fmt::Display for BuildType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => write!(f, "Debug"),
            Self::Release => write!(f, "Release"),
        }
    }
}
