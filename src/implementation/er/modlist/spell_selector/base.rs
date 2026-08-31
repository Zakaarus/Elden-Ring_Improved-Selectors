use std::sync::LazyLock;
use anyhow::anyhow;

use super::super::action_reader::register_bindings;
use crate::attempt;
use super::internal::{action, cast_slot::request_actions};
use crate::settings::Config;
use super::super::super::utils::{get_main_player,change_spell};
use super::super::GameMod;
use super::begin_slot;
use super::end_slot;


use fromsoftware_shared::{Program, SharedTaskImpExt};
use eldenring::cs::{CSTaskGroupIndex, CSTaskImp};
use std::time::Duration;
use eldenring::fd4::FD4TaskData;
use eldenring::util::system::wait_for_system_init;

/* <=====================================================================================================================================> */

pub const MOD:GameMod = GameMod  
{
    context:"spell_selector",
    init
};

static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new(MOD.context));
#[expect(clippy::field_scoped_visibility_modifiers, reason="No special logic needed for getting/setting, both true and false are unconditionally usable.")]
pub(super) struct Settings
{
    pub(super) auto_refresh:bool,
    pub(super) no_miscast:bool,
    pub(super) cast_slot:bool,
}
pub(super) static SETTINGS: LazyLock<Settings> = LazyLock::new
(||{
    let auto_refresh = CONFIG.deep_query(&["auto_refresh"])
        .and_then(toml::Value::as_bool)
        .unwrap_or_default();
    let no_miscast = CONFIG.deep_query(&["no_miscast"])
        .and_then(toml::Value::as_bool)
        .unwrap_or_default();
    let cast_slot = CONFIG.deep_query(&["cast_slot"])
        .and_then(toml::Value::as_bool)
        .unwrap_or_default();
    return Settings
    {
        auto_refresh,
        no_miscast,
        cast_slot
    }
});

/* <=====================================================================================================================================> */

fn init()
{
    wait_for_system_init(&Program::current(), Duration::MAX)
        .unwrap_or_else(|error|panic!("Entry Point - System Init Wait: {error}"));

    register_bindings(&CONFIG, action);

    let cs_task = CSTaskImp::wait_for_instance(Duration::from_secs(2))
        .unwrap_or_else(|error|panic!("Entry Point - CS Task Imp: {error}"));
    
    cs_task.run_recurring(frame_begin,CSTaskGroupIndex::FrameBegin);
    cs_task.run_recurring(frame_end,CSTaskGroupIndex::FrameEnd);
    cs_task.run_recurring(chr_ins_pre_behavior_safe, CSTaskGroupIndex::ChrIns_PreBehaviorSafe);
}

fn chr_ins_pre_behavior_safe(_data:&FD4TaskData)
    {request_actions();}

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
    {["no end slot", "Static object not found: WorldChrMan", "Main Player not found."] ("Spell Selector Frame End")
        change_spell(Some(get_main_player()?),end_slot()
            .ok_or_else(||return anyhow!("no end slot"))?);
    };
}

/* <=====================================================================================================================================> */

