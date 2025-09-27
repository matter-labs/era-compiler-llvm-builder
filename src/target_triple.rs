//!
//! The ZKsync LLVM target triples.
//!

///
/// The list of target triples used as constants.
///
/// It must be in the lowercase.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetTriple {
    /// The EraVM back end developed by Matter Labs.
    EraVM,
    /// The EVM back end developed by Matter Labs.
    EVM,
}

impl std::str::FromStr for TargetTriple {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "eravm" => Ok(Self::EraVM),
            "evm" => Ok(Self::EVM),
            value => Err(format!("Unsupported target triple: `{value}`")),
        }
    }
}

impl std::fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EraVM => write!(f, "eravm"),
            Self::EVM => write!(f, "evm"),
        }
    }
}
