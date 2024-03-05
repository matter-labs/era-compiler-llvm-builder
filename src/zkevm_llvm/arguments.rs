//!
//! The zkEVM LLVM builder arguments.
//!

use structopt::StructOpt;

///
/// The zkEVM LLVM builder arguments.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "llvm-builder", about = "The zkEVM LLVM framework builder")]
pub enum Arguments {
    /// Clone the branch specified in `LLVM.lock`.
    Clone,
    /// Build the LLVM framework.
    Build {
        /// Whether to build the 'Debug' version.
        #[structopt(long = "debug")]
        debug: bool,
        /// Additional targets to build LLVM with.
        #[structopt(long = "targets", multiple = true)]
        targets: Vec<String>,
        /// Whether to build the LLVM tests.
        #[structopt(long = "enable-tests")]
        enable_tests: bool,
        /// Whether to build LLVM for source-based code coverage.
        #[structopt(long = "enable-coverage")]
        enable_coverage: bool,
        /// Extra arguments to pass to CMake.  
        /// A leading backslash will be unescaped.
        #[structopt(long = "extra-args", multiple = true)]
        extra_args: Vec<String>,
        /// Whether to use compiler cache (ccache) to speed-up builds.
        #[structopt(long = "use-ccache")]
        use_ccache: bool,
    },
    /// Checkout the branch specified in `LLVM.lock`.
    Checkout {
        /// Remove all artifacts preventing the checkout (removes all local changes!).
        #[structopt(long = "force")]
        force: bool,
    },
    /// Clean the build artifacts.
    Clean,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
