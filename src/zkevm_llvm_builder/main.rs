//!
//! The zkEVM LLVM builder.
//!

pub(crate) mod arguments;

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

    // compiler_llvm_builder::clone(
    //     "https://github.com/matter-labs/compiler-llvm",
    //     arguments.branch.as_str(),
    // )?;
    compiler_llvm_builder::build()?;

    Ok(())
}
