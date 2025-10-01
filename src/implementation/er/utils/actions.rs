use eldenring_util::input::is_key_pressed;

use crate::settings::Config;

/// array of bindings -> array of actions which are active this frame
pub fn actions_this_frame(bindings:&[(String,Vec<String>)]) 
    -> Vec<&str>
{ //I'm failing somewhere here? ACTION_BINDINGS has the bindings. But printing the array passed to internal::spell_select() shows the input isn't picked up.
//nvm? It seems to solve itself when I don't have overlapping binds? I'll need to fix it later
    return bindings.iter()
        .filter
        (|action_bind| 
            return action_bind.1.iter()
                .all(|input| return is_input_pressed(input))
        )
        .map(|action_bind| return action_bind.0.as_str())
        .collect();
}

fn is_input_pressed(input:&str) 
    -> bool
{
    return i32::from_str_radix
    (
        input.strip_prefix("0x")
            .unwrap_or(input), 16
    )
        .map_or_else
        (
            |_| 
            {
                return is_key_pressed(9001); //idk yet
            },
            |key| return is_key_pressed(key)
        );
}

/* <==========================================================================================================================> */

/// Config -> actions bindings list
pub fn get_action_bindings(config:&Config) 
    -> Vec<(String, Vec<String>)> //I know I shouldn't use a tuple but I'll fix it eventually
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