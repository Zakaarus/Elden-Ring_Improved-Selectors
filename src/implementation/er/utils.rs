use keyboard_types::Code;
use windows::Win32::UI::Input::KeyboardAndMouse;
use eldenring::cs::{PlayerIns, WorldChrMan};
use eldenring_util::{input::is_key_pressed, singleton::get_instance};
use fromsoftware_shared::OwnedPtr;

use crate::settings::Config;

pub fn get_world_chr_man() 
    -> Option<&'static mut WorldChrMan>
{
    //SAFETY: See get_instance
    return unsafe { get_instance::<WorldChrMan>() }.ok()?;
}

pub const fn get_main_player(world_chr_man:&'static mut WorldChrMan) 
    -> Option<&'static mut OwnedPtr<PlayerIns>>
{
    return world_chr_man.main_player
        .as_mut();
}

fn is_bind_pressed(bind:&[String]) 
    -> bool
{
    return bind.iter()
        .all(|input|return is_input_pressed(input));
}

fn is_input_pressed(input:&String) 
    -> bool
{
    match input
    {
        _ => {}
    }
    false
}

pub fn actions_this_frame(bindings:&[(String,Vec<String>)]) -> Vec<&String>
{
    return bindings.iter()
        .filter(|action_bind| return is_bind_pressed(&action_bind.1))
        .map(|action_bind| return &action_bind.0)
        .collect();
}

/// Config -> actions bindings list
pub fn get_action_bindings(config:&Config) 
    -> Vec<(String, Vec<String>)>
{
    return config.deep_query(&["controls"])
        .and_then(|table| return table.as_table() )
        .map
        (|table| 
            return table.iter()
                .map
                (|(key,bind)| 
                    return /*Tuple*/
                    (
                        key.to_owned(),
                        bind.as_array()
                            .map
                            (|bind_exists| 
                                return bind_exists.iter()
                                    .filter_map(|input| return Some(input.as_str()?.to_owned()))
                                    .collect::<Vec<String>>()
                            )
                            .unwrap_or_default()
                    )
                )
            .collect::<Vec<(String,Vec<String>)>>()
        )
        .unwrap_or_default();
}