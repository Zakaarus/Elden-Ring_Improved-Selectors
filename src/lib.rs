//!Elden Ring Mod? I'm not sure what to put here yet

///Dllmain is where the C externs for running DLLs are defined. 
mod DllMain;
///implementation is where the effects of the DLL are defined.
mod implementation;
use implementation::entry_point;
use implementation::panic_hook;
///settings is where settings are kept
mod settings;
