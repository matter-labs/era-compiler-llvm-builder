//!
//! The target environments to build LLVM.
//!

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

impl std::str::FromStr for TargetEnv {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "gnu" => Ok(Self::GNU),
            "musl" => Ok(Self::MUSL),
            value => Err(format!("Unsupported target environment: `{}`", value)),
        }
    }
}

impl std::fmt::Display for TargetEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GNU => write!(f, "gnu"),
            Self::MUSL => write!(f, "musl"),
        }
    }
}
