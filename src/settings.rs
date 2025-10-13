use std::{ffi::OsString, fs, os::{raw::c_void, windows::ffi::OsStringExt}, path::PathBuf};
use toml::Value;
use windows::Win32::{Foundation::{HMODULE, MAX_PATH}, System::LibraryLoader::GetModuleFileNameW};
use std::sync::LazyLock;
use anyhow::anyhow;
mod default;
use default::DEFAULT_SETTINGS_RAW;

static DEFAULTS: LazyLock<Value> = LazyLock::new(||return toml::from_str(DEFAULT_SETTINGS_RAW).expect("Defaults Init: Invalid"));

unsafe extern "C" {static mut __ImageBase: c_void;}
static FULL_CONFIG: LazyLock<Value> = LazyLock::new
(||{
    let file_name_array:&mut [u16;MAX_PATH as usize] = &mut [0;MAX_PATH as usize];
    //SAFETY: ...C.
    let _:u32 = unsafe
    {
        GetModuleFileNameW
        ( 
            Some(HMODULE(&raw mut __ImageBase)), 
            file_name_array
        ) 
    };
    return PathBuf::from(OsString::from_wide(file_name_array))
        .parent()
        .ok_or_else(|| return anyhow!("The DLL doesn't have a parent directory?!"))
        .map(|dll_dir| return fs::read_to_string(dll_dir.join("config.toml")))
        .and_then
        (|read_results| 
            return Ok
            (
                read_results.map
                (|file_contents| 
                    return Ok(toml::from_str::<Value>(&file_contents)?)
                )?
            )
        )
        .flatten()
        .unwrap_or_else
        (|error|{
            println!("Full Config: {error}. Using defaults...");
            return DEFAULTS.to_owned();
        });
});

/* <========================================================================================================================================> */

/// Thread/context local copies of a Value, taken from both `FULL_CONFIG` and DEFAULTS.
/// The context is the name of said Value.
/// Use them as-is or deserialise into more specialised structs
pub struct Config
{
    pub defaults:Option<Value>,
    pub file:Option<Value>
}

impl Config
{
    /// Name of the Value -> Config of that Value
    pub fn new(context:&'static str) 
        -> Self
    {
        return Self
        {
            defaults:FULL_CONFIG.get(context).cloned(),
            file:DEFAULTS.get(context).cloned()
        };
    }

    /// Nested Value's path as an array of strings -> Value from lookup in Config
    pub fn deep_query(&self,name:&[&str])
        -> Option<&Value>
    {
        return 
        [
            self.file.as_ref(),
            self.defaults.as_ref()
        ].iter()
            .find_map
            (|&table|{
                return name.iter().try_fold
                (
                    table?,
                    |entry,key|{
                        return entry.get(key)
                    }
                )
            });
    }
}