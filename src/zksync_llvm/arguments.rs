//!
//! The ZKsync LLVM builder arguments.
//!

use structopt::StructOpt;

///
/// The ZKsync LLVM builder arguments.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "llvm-builder", about = "The ZKsync LLVM framework builder")]
pub enum Arguments {
    /// Clone the branch specified in `LLVM.lock`.
    Clone {
        /// Clone with full commits history.
        #[structopt(long)]
        deep: bool,
        /// Target environment to build LLVM (GNU or MUSL).
        #[structopt(long = "target-env", default_value = "gnu")]
        target_env: compiler_llvm_builder::platforms::TargetEnv,
    },
    /// Build the LLVM framework.
    Build {
        /// Whether to build the 'Debug' version.
        #[structopt(long = "debug")]
        debug: bool,
        /// Target environment to build LLVM (`gnu` or `musl`).
        #[structopt(long = "target-env", default_value = "gnu")]
        target_env: compiler_llvm_builder::platforms::TargetEnv,
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
        /// Whether to build with assertions enabled or not.
        #[structopt(long = "enable-assertions")]
        enable_assertions: bool,
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
