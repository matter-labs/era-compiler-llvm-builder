//!
//! The ZKsync LLVM builder platforms.
//!

pub mod aarch64_linux_gnu;
pub mod aarch64_linux_musl;
pub mod aarch64_macos;
pub mod shared;
pub mod x86_64_linux_gnu;
pub mod x86_64_linux_musl;
pub mod x86_64_macos;
pub mod x86_64_windows_gnu;

use std::str::FromStr;

///
/// The list of platforms used as constants.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    /// The native X86 platform.
    X86,
    /// The native AArch64 platform.
    AArch64,
    /// The EraVM back end developed by Matter Labs.
    EraVM,
    /// The EVM back end developed by Matter Labs.
    EVM,
}

///
/// The list of target environments used as constants.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetEnv {
    /// The GNU target environment.
    GNU,
    /// The MUSL target environment.
    MUSL,
}

impl FromStr for TargetEnv {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "gnu" => Ok(Self::GNU),
            "musl" => Ok(Self::MUSL),
            value => Err(format!("Unsupported target environment: `{}`", value)),
        }
    }
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "EraVM" => Ok(Self::EraVM),
            "EVM" => Ok(Self::EVM),
            value => Err(format!("Unsupported platform: `{}`", value)),
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::X86 => write!(f, "X86"),
            Self::AArch64 => write!(f, "AArch64"),
            Self::EraVM => write!(f, "EraVM"),
            Self::EVM => write!(f, "EVM"),
        }
    }
}
