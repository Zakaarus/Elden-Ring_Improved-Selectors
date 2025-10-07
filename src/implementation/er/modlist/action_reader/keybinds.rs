use std::sync::{LazyLock, Mutex, OnceLock};

use crate::settings::Config;

pub static KEYBIND_BUFFER: LazyLock<Mutex<Vec<Keybind>>> = LazyLock::new(|| return Mutex::new(Vec::new()));
pub static KEYBINDS: OnceLock<Vec<Keybind>> = OnceLock::new();
#[derive(Clone)]
pub struct Keybind
{
    pub action:&'static str,
    pub bind:Vec<i32>,
    pub callback:fn(&str)
} 

#[flux_rs::trusted] //I simply don't know how to fix this one, help. Lifetime management is my WEAKNESS.
pub fn register_bindings(config:&'static Config, callback:fn(&str))
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
                                    .filter_map(|input| return input.as_integer()?.try_into().ok())
                                    .collect::<Vec<i32>>()
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
        #[expect(clippy::arithmetic_side_effects, reason = "Warning: usize to negated isize shenanigans.")]
        return -(TryInto::<isize>::try_into(keybind.bind.len())
            .unwrap_or_else(|error|panic!("isize CAST ERROR: {error:}")));
    });

    let mut keybinds = KEYBIND_BUFFER.lock()
        .unwrap_or_else(|error|panic!("MUTEX LOCK ERROR?!: {error:#?}"));
    keybinds.append(&mut action_bindings);
}