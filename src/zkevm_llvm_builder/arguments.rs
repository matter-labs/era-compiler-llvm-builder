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
    /// The zkEVM LLVM framework branch or tag.
    #[structopt(long = "branch")]
    pub branch: String,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
