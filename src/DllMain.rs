#![allow(non_snake_case,reason="C externs have their own naming conventions.")]

use std::panic;
use core::ffi::c_void;
use std::ptr::null_mut;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::Threading::QueueUserWorkItem;
use windows::Win32::System::Threading::WT_EXECUTEDEFAULT;
use crate::entry_point;
use crate::panic_hook;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HMODULE, reason: u32) 
    -> bool
{
    if reason != 1 {return true;}
    #[cfg(debug_assertions)] set_panic_hook();
    //SAFETY: ...C.
    unsafe 
    {
        DisableThreadLibraryCalls(hmodule)
            .unwrap_or_else(|error|println!("Warn: DisableThreadLibraryCalls failed. Error: {error}"));
        QueueUserWorkItem(Some(dll_thread), Some(null_mut()), WT_EXECUTEDEFAULT)
            .unwrap_or_else(|error|panic!("FAILED TO START dll_thread ERROR: {error}"));
    }
    return true;
}

unsafe extern "system" fn dll_thread(_:*mut c_void) 
    -> u32
{
    entry_point(); 
    return 1;
}


/// Set the panic hook.
/// The panic hook is a function that takes in `&std::panic::PanicHookInfo`.
/// Flux is incompatible with the box shenanigans needed to set a panic hook.
#[flux_rs::trusted] 
#[cfg(debug_assertions)]
fn set_panic_hook()
{
    panic::set_hook(Box::new(panic_hook));
}