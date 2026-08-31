#![expect(stable_features, reason = "Flux's version still considers the following features unstable.")]//bruh flux is outdated on the compiler version.
#![feature(rust1)] //just so flux doesn't warn
#![feature(never_type)]
#![feature(if_let_guard)] 

//!Elden Ring mod for reworking the item/spell/weapon selection controls.

/// Dllmain is where the C externs for running DLLs are defined. 
mod DllMain;
/// implementation is where the effects of the DLL are defined.
mod implementation;
use implementation::entry_point;
use implementation::panic_hook;
/// settings is where settings are kept.
mod settings;
