use std::sync::LazyLock;
use crate::settings::Config;

use super::modlist::MOD_LIST;

static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new("general"));

/// This is what runs when `DllMain` makes its thread.
pub fn entry_point() 
{
    for game_mod 
        in MOD_LIST.iter()
            .filter
            (|game_mod| {
                return CONFIG
                    .deep_query(&["enabled",game_mod.context])
                    .and_then(|enabled| return enabled.as_bool())
                    .unwrap_or(true);}
            )
        {(game_mod.init)();}
}
