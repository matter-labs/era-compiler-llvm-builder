//!
//! The zkEVM LLVM builder arguments.
//!

use structopt::StructOpt;

///
/// The zkEVM LLVM builder arguments.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "llvm-builder", about = "The zkEVM LLVM framework builder")]
pub struct Arguments {
    /// Whether to build the 'Release' version.
    #[structopt(long = "release")]
    pub release: bool,

    /// Clean the build artifacts and exit.
    #[structopt(long = "clean")]
    pub clean: bool,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
