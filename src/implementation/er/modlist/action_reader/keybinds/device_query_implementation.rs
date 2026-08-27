use std::str::FromStr;

use device_query::Keycode;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyName
{
    Keycode(Keycode),
    Other(String),
    //Unknown
}

impl From<&str>
    for KeyName
{
    fn from(value: &str) 
        -> Self 
    {
        return Keycode::from_str(value)
            .inspect_err(|error|println!("{value:} TO KEYBOARD Keycode ERROR: {error:}. Trying as mouse input."))
            .map_or_else(|_|return Self::Other(value.to_owned()), Self::Keycode);
    }
}