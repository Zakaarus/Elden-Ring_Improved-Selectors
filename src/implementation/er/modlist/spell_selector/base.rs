use std::sync::LazyLock;
use anyhow::anyhow;
use eldenring::fd4::FD4TaskData;

use super::super::action_reader::register_bindings;
use crate::attempt;
use crate::implementation::er::modlist::spell_selector::internal::action;
use crate::settings::Config;
use super::super::super::utils::{get_main_player,change_spell};
use super::super::ERMod;
use super::begin_slot;
use super::end_slot;

/* <=====================================================================================================================================> */

pub const MOD:ERMod = ERMod  
{
    context:"spell_selector",
    frame_begin,
    frame_end, 
    init
};

static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new(MOD.context));
#[expect(clippy::field_scoped_visibility_modifiers, reason="No special logic needed for getting/setting, both true and false are unconditionally usable.")]
pub(super) struct Settings
{
    pub(super) auto_refresh:bool,
    pub(super) no_miscast:bool
}
pub(super) static SETTINGS: LazyLock<Settings> = LazyLock::new
(||{
    let auto_refresh = CONFIG.deep_query(&["auto_refresh"])
        .and_then(toml::Value::as_bool)
        .unwrap_or_default();
    let no_miscast = CONFIG.deep_query(&["no_miscast"])
        .and_then(toml::Value::as_bool)
        .unwrap_or_default();
    return Settings
    {
        auto_refresh,
        no_miscast
    }
});

/* <=====================================================================================================================================> */

fn init(){register_bindings(&CONFIG, action);}

fn frame_begin(_data:&FD4TaskData)
{
    attempt!
    {["no begin slot", "Static object not found: WorldChrMan", "Main Player not found."] ("Spell Selector Frame Begin")
        change_spell(Some(get_main_player()?),begin_slot()
            .ok_or_else(||return anyhow!("no begin slot"))?);
    };
}

fn frame_end(_data:&FD4TaskData)
{
    attempt!
    {["Static object not found: WorldChrMan", "Main Player not found."] ("Spell Selector Frame End")
        change_spell(Some(get_main_player()?),end_slot());
    };
}

/* <=====================================================================================================================================> */

