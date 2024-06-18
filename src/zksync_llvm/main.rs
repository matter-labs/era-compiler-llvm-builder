//!
//! The ZKsync LLVM builder.
//!

pub(crate) mod arguments;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;

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
    if env::var("VERBOSE").is_ok() {
        println!("std::env::args(): {:#?}", std::env::args());
        println!(
            "\nstd::env::args() collected: {}",
            std::env::args().collect::<Vec<_>>().join(" ")
        );
    }
    let arguments = Arguments::new();

    match arguments {
        Arguments::Clone { deep, target_env } => {
            let lock = compiler_llvm_builder::Lock::try_from(&PathBuf::from("LLVM.lock"))?;
            compiler_llvm_builder::clone(lock, deep, target_env)?;
        }
        Arguments::Build {
            debug,
            target_env,
            targets,
            enable_tests,
            enable_coverage,
            extra_args,
            use_ccache,
            enable_assertions,
            sanitizer,
        } => {
            let build_type = compiler_llvm_builder::BuildType::from(debug);

            let mut targets = targets
                .into_iter()
                .map(|target| compiler_llvm_builder::Platform::from_str(target.as_str()))
                .collect::<Result<HashSet<compiler_llvm_builder::Platform>, String>>()
                .map_err(|platform| anyhow::anyhow!("Unknown platform `{}`", platform))?;
            targets.insert(compiler_llvm_builder::Platform::EraVM);
            targets.insert(compiler_llvm_builder::Platform::EVM);

            let extra_args_unescaped: Vec<String> = extra_args
                .iter()
                .map(|argument| {
                    argument
                        .strip_prefix('\\')
                        .unwrap_or(argument.as_str())
                        .to_owned()
                })
                .collect();
            if env::var("VERBOSE").is_ok() {
                println!("\nextra_args: {:#?}", extra_args);
                println!("\nextra_args_unescaped: {:#?}", extra_args_unescaped);
            }

            if use_ccache {
                compiler_llvm_builder::utils::check_presence("ccache")?;
            }

            compiler_llvm_builder::build(
                build_type,
                target_env,
                targets,
                enable_tests,
                enable_coverage,
                extra_args_unescaped,
                use_ccache,
                enable_assertions,
                sanitizer,
            )?;
        }
        Arguments::Checkout { force } => {
            let lock = compiler_llvm_builder::Lock::try_from(&PathBuf::from("LLVM.lock"))?;
            compiler_llvm_builder::checkout(lock, force)?;
        }
        Arguments::Clean => {
            compiler_llvm_builder::clean()
                .with_context(|| "Unable to remove target LLVM directory")?;
        }
    }

    Ok(())
}
