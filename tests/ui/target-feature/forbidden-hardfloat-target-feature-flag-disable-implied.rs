//@ compile-flags: --target=x86_64-unknown-linux-gnu --crate-type=lib
//@ needs-llvm-components: x86
//@ compile-flags: -Ctarget-feature=-sse
// For now this is just a warning.
//@ build-pass
#![feature(no_core, lang_items)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}
