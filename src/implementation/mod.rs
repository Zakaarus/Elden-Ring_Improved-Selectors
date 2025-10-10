mod er;
pub use er::entry_point;

use std::panic::PanicHookInfo;
use std::{thread, time};
/// Allow panic error to appear in console
pub fn panic_hook(error: &PanicHookInfo)
{
    println!("ERROR: {error}");
    eprintln!("{error}");
    thread::sleep(time::Duration::from_millis(5000));
}