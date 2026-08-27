use std::{ops::Neg, sync::{LazyLock, Mutex, OnceLock}};

use serde::Deserialize;
use toml::{Value, map::Map};

use crate::settings::Config;
use super::KeyName;

pub static KEYBIND_BUFFER: LazyLock<Mutex<Vec<Keybind>>> = LazyLock::new(|| return Mutex::new(Vec::new()));
pub static KEYBINDS: OnceLock<Box<[Keybind]>> = OnceLock::new();

type Callback = fn(&str);
pub struct Keybind
{
    pub action:String,
    pub callback:Callback,//fn(&str),
    pub bind:Box<[Key]>
} 

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct Modifiers
{
    // ^ ->  Released
    // _ ->  Pressed
    // ! ->  Not Held
    // Default:  Held
    pub key_state:KeyState  //idk how to represent button states
}
impl From<String> 
    for Modifiers
{
    fn from(modifier_string:String) 
        -> Self
    {
        let modifier_array = modifier_string.chars()
            .filter_map
            (|character|
                return match character
                {
                    '^' => Some(Modifier::KeyState(KeyState::Released)),
                    '_' => Some(Modifier::KeyState(KeyState::Pressed)),
                    '!' => Some(Modifier::KeyState(KeyState::NotHeld)),
                    //'$' => Some(Modifier::TestModifier(TestModifier::Test1())),
                    _ => None
                }
            )
            .collect::<Vec<Modifier>>();

        return Self
        { 
            key_state: modifier_array.iter()
                .find_map
                (|modifier|
                    return if let Modifier::KeyState(key_state) = *modifier 
                        {Some(key_state)} 
                    else 
                        {None}
                )
                .unwrap_or_default(),
            //..Default::default() 
        };
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum KeyState
{
    Released,
    Pressed,
    NotHeld,
    #[default]
    Held
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum Modifier
{
    KeyState(KeyState),
    Unknown
    //TestModifier(TestModifier)
}
/*pub enum TestModifier
{
    Test1()
}*/

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Key
{
    pub modifiers:Modifiers,
    pub name:KeyName,
    pub event:KeyEvent
}

impl From<&str> 
    for Key 
{
    fn from(key_string:&str) 
        -> Self 
    {
        let (stripped_key, stripped_modifiers) : (String,String)= key_string
            .chars()
            .partition
            (|character|
                return character.is_alphanumeric() 
                    || character == &':' 
                    || character == &'{' 
                    || character == &'}' 
                    || character == &'"' 
                    || character == &'='
            );
        let modifiers = stripped_modifiers.into();
        let name: KeyName = stripped_key.as_str().into();
        return Self
        {
            modifiers,
            name:name.clone(),
            event:KeyEvent
            {
                name,
                state:KeyEvent::state_match(modifiers.key_state)
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyEvent
{
    pub name:KeyName,
    pub state:KeyState //Only ever Pressed or Released, never Held or NotHeld.
    //I know I should make a new type that is like a keystate but restricted to pressed and released, but I'm too lazy
}
impl KeyEvent 
{
    pub const fn state_match(input: KeyState)
        -> KeyState
    {
        return match input
        {
            KeyState::Released | KeyState::NotHeld => KeyState::Released,
            KeyState::Pressed | KeyState::Held => KeyState::Pressed
        }
    }
}










pub fn register_bindings(config:&'static Config, callback:Callback)
{
    let table_to_keybinds = |action_binds_table:&Map<String,Value>| 
        -> Vec<Keybind>
    {
        let value_string_to_key_array = |bind: &Value|
            -> Box<[Key]>
        {
            return bind.as_str()
                .unwrap_or_default()
                .split(' ')
                .map(Key::from)
                .collect()
        };

        return action_binds_table.iter()
            .map
            (|(action,all_binds)|
                return 
                (
                    action,
                    all_binds.as_array()
                        .map
                        (|bind|
                            return bind.iter()
                                .map(value_string_to_key_array)
                                .collect::<Vec<Box<[Key]>>>()
                        )
                        .unwrap_or_default()
                )
            )
            .flat_map
            (|(action,all_binds)|
                return all_binds.into_iter()
                    .map
                    (move |bind|
                        return Keybind
                        {
                            action:action.clone(),
                            callback,
                            bind
                        }
                    )
            )
            .collect::<Vec<Keybind>>();
    };




    let mut action_bindings = config.deep_query(&["controls"])
        .and_then(|action_binds_table| return action_binds_table.as_table())
        .map(table_to_keybinds)
        .unwrap_or_default();

    action_bindings.sort_unstable_by_key
    (|keybind|{
        return TryInto::<isize>::try_into(keybind.bind.len())
            .map_or_else
            (
                |error|panic!("Binding Registry - Sort: {error:}"),
                Neg::neg
            );
    });

    let mut keybinds = KEYBIND_BUFFER.lock()
        .unwrap_or_else(|error|panic!("Binding Registry - Keybind Buffer Mutex: {error:#?}"));
    keybinds.append(&mut action_bindings);

}