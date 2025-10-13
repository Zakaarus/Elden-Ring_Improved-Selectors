use std::{num::TryFromIntError, ops::Neg, sync::{LazyLock, Mutex, OnceLock}};
use anyhow::anyhow;

use crate::{implementation::handle_error, settings::Config};

pub static KEYBIND_BUFFER: LazyLock<Mutex<Vec<Keybind>>> = LazyLock::new(|| return Mutex::new(Vec::new()));
pub static KEYBINDS: OnceLock<Box<[Keybind]>> = OnceLock::new();
#[derive(Clone)]
pub struct Keybind
{
    pub action:&'static str,
    pub bind:Box<[i32]>,
    pub callback:fn(&str)
} 

#[flux_rs::trusted] //I simply don't know how to fix this one, help. Lifetime management is my WEAKNESS.
pub fn register_bindings(config:&'static Config, callback:fn(&str)) //Maybe I should make this return a result but I'm not sure, since its success is so crucial.
{
    let mut action_bindings = config.deep_query(&["controls"])
        .and_then(|table| return table.as_table() )
        .map
        (|table| 
            return table.iter()
                .map
                (|(action,bind)| 
                    return Keybind
                    {
                        action:action.as_str(),
                        bind:bind.as_array()
                            .map
                            (|bind_exists| 
                                return bind_exists.iter()
                                    .filter_map
                                    (|input| 
                                        return input.as_integer()?
                                            .try_into()
                                            .inspect_err
                                            (|error:&TryFromIntError|{
                                                handle_error::<()>
                                                (
                                                    Err(anyhow!(error.to_string())),
                                                    "Binding Registry",
                                                    &[]
                                                );
                                            })
                                            .ok()
                                    )
                                    .collect::<Box<[i32]>>()
                            )
                            .unwrap_or_default(),
                        callback
                    }
                )
                .collect::<Vec<Keybind>>()  
        )
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