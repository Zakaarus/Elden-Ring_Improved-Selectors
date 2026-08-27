
use anyhow::Result;
use std::{thread, time::Duration, panic::PanicHookInfo};

/// Allow panic error to appear in console.
pub fn panic_hook(error: &PanicHookInfo)
{
    println!("ERROR: {error}");
    eprintln!("{error}");
    thread::sleep(Duration::from_secs(5));
}

/// attempt!{func} where func is the body of a closure that returns `anyhow::Result`<()>
/// This allows the use of ? in functions that return (). `attempt_success` = true if (), false if None.
/// 
/// ```
/// attempt!
/// {["ignore","ignore"]("context") 
///     ...
/// }
/// ```
/// where `ignore` is any string you want the error handler not to process, `context` is a string that the error handler will use to help identify where the error was thrown from.
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

/// Custom error handling implementation.
pub fn handle_error<T>(result:Result<T>,context:&str,ignore:&[&str])
    -> Option<T>
{
    return match result 
    {
        Ok(success) => Some(success),
        Err(error) => {
            if !ignore.contains(&AsRef::<str>::as_ref(&error.to_string()))
            {
                println!("{context}: {error:#}");
            }
            return None;
        }
    }
}