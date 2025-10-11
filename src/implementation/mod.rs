mod er;
use anyhow::Result;
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

/// attempt!{func} where func is the body of a closure that returns `anyhow::Result`<()>
/// This allows the use of ? in functions that return (). `attempt_success` = true if (), false if None
/// 
/// Use attempt!{[ignore] func} where [ignore] is anything you don't want to handle.
/// Returns Option<()> where None is no error, () is some error that has already been handled.
#[macro_export] 
macro_rules! attempt
{
    {[$($ignore:expr),*] ($($context:expr)*) $($function:tt)*}=>
    {{
        let attempt_result = 
        (|| -> anyhow::Result<()>{
            $($function)*
            return Ok(());
        })();
        $crate::implementation::handle_error::<()>
        (
            attempt_result,
            $($context)*,
            &[$($ignore),*]
        );
    }};
    
    {($($context:expr)*) $($function:tt)*} => 
    {{
        let attempt_result = 
        (|| -> anyhow::Result<()> {
                $($function)*
                return Ok(());
        })();
        $crate::implementation::handle_error::<()>
        (
            attempt_result,
            $($context)*,
            &[]
        );
    }};
}

/// Custom error handling implementation
pub fn handle_error<T>(result:Result<T>,context:&str,ignore:&[&str])
    -> Option<T>
{
    match result 
    {
        Ok(success) => return Some(success),
        Err(error) => {
            if !ignore.contains(&AsRef::<str>::as_ref(&error.to_string()))
            {
                println!("{context}: {error:#}");
            }
            return None;
        }
    }
}