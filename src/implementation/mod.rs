mod er;
pub use er::entry_point;

#[cfg(debug_assertions)]
use std::panic::PanicHookInfo;
#[cfg(debug_assertions)]
use std::{thread, time};
///Allow panic error to appear in console
#[cfg(debug_assertions)]
pub fn panic_hook(error: &PanicHookInfo)
{
    println!("ERROR: {error}");
    eprintln!("{error}");
    thread::sleep(time::Duration::from_millis(5000));
}