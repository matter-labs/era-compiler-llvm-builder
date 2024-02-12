//!
//! The zkEVM LLVM builder lock file.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;

///
/// The lock file data.
///
/// This file describes the exact reference of the LLVM framework.
///
#[derive(Debug, Deserialize)]
pub struct Lock {
    /// The LLVM repository URL.
    pub url: String,
    /// The LLVM repository branch.
    pub branch: Option<String>,
    /// The LLVM repository commit reference.
    pub r#ref: Option<String>,
}

impl TryFrom<&PathBuf> for Lock {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut config_str = String::new();
        let mut config_file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                anyhow::bail!("Error opening {:?} file: {}", path, err);
            }
        };
        config_file.read_to_string(&mut config_str)?;
        Ok(toml::from_str(&config_str)?)
    }
}
