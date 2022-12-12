//!
//! The zkEVM LLVM builder.
//!

pub(crate) mod arguments;

use std::path::PathBuf;

use self::arguments::Arguments;

///
/// The entry.
///
fn main() {
    main_wrapper().expect("LLVM builder error");
}

///
/// The entry result wrapper.
///
fn main_wrapper() -> anyhow::Result<()> {
    let arguments = Arguments::new();

    let lock = compiler_llvm_builder::Lock::try_from(&PathBuf::from("LLVM.lock"))?;

    compiler_llvm_builder::clone(lock)?;
    compiler_llvm_builder::build()?;

    Ok(())
}
