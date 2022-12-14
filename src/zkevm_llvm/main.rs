//!
//! The zkEVM LLVM builder.
//!

pub(crate) mod arguments;

use std::path::PathBuf;

use self::arguments::Arguments;

/// The default path to the LLVM lock file.
pub const LLVM_LOCK_DEFAULT_PATH: &str = "LLVM.lock";

///
/// The entry.
///
fn main() {
    match main_inner() {
        Ok(()) => std::process::exit(0),
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1)
        }
    }
}

///
/// The entry result wrapper.
///
fn main_inner() -> anyhow::Result<()> {
    let arguments = Arguments::new();

    match arguments {
        Arguments::Clone => {
            let lock = compiler_llvm_builder::Lock::try_from(&PathBuf::from("LLVM.lock"))?;
            compiler_llvm_builder::clone(lock)?;
        }
        Arguments::Build { debug } => {
            let build_type = compiler_llvm_builder::BuildType::from(debug);
            compiler_llvm_builder::build(build_type)?;
        }
        Arguments::Checkout { force } => {
            let lock = compiler_llvm_builder::Lock::try_from(&PathBuf::from("LLVM.lock"))?;
            compiler_llvm_builder::checkout(lock, force)?;
        }
        Arguments::Clean => {
            compiler_llvm_builder::clean()?;
        }
    }

    Ok(())
}
