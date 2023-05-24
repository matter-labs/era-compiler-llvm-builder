//!
//! The zkEVM LLVM builder.
//!

pub(crate) mod arguments;

use std::path::PathBuf;
use snailquote::unescape;


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
            eprintln!("Error: {error:?}");
            std::process::exit(1)
        }
    }
}

///
/// The entry result wrapper.
///
fn main_inner() -> anyhow::Result<()> {
    println!("\nstd::env::args() collected: {}", std::env::args().collect::<Vec<_>>().join(" "));
    println!("std::env::args(): {:#?}", std::env::args());
    let arguments = Arguments::new();

    match arguments {
        Arguments::Clone => {
            let lock = compiler_llvm_builder::Lock::try_from(&PathBuf::from("LLVM.lock"))?;
            compiler_llvm_builder::clone(lock)?;
        }
        Arguments::Build {
            debug,
            enable_tests,
            extra_args, 
        } => {
            println!("\nextra_args: {:#?}", extra_args);
            let extra_args_unescaped:Vec<_> = extra_args.iter()
                .map(|s| unescape(s))
                .collect::<Result<_, _>>()
                .unwrap();
            println!("\nextra_args_unescaped: {:#?}", extra_args_unescaped);
            let build_type = compiler_llvm_builder::BuildType::from(debug);
            compiler_llvm_builder::build(build_type, enable_tests, extra_args_unescaped)?;
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
