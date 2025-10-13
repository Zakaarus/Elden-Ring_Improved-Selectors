use std::{sync::LazyLock, time::Duration};
use eldenring::{cs::{CSTaskGroupIndex, CSTaskImp}, fd4::FD4TaskData};
use eldenring_util::{program::Program, singleton::get_instance, system::wait_for_system_init, task::CSTaskImpExt};
use crate::settings::Config;

use super::modlist::MOD_LIST;

static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new("general"));

/// This is what runs when `DllMain` makes its thread
pub fn entry_point() 
{
    wait_for_system_init(&Program::current(), Duration::MAX)
        .unwrap_or_else(|error|panic!("Entry Point - System Init Wait: {error}"));



    //SAFETY: See get_instance
    let cs_task = unsafe 
    { 
        get_instance::<CSTaskImp>()
            .unwrap_or_else(|error|panic!("Entry Point - CS Task Imp: {error}"))
            .expect("Entry Point CS Task Imp: Returned None.") 
    };

    for er_mod 
        in MOD_LIST.iter()
            .filter
            (|er_mod| 
                return CONFIG
                    .deep_query(&["enabled",er_mod.context])
                    .and_then(|enabled| return enabled.as_bool())
                    .unwrap_or(true)
            )
    {
        (er_mod.init)();
        cs_task.run_recurring
        (
            |data: &FD4TaskData| {(er_mod.frame_begin)(data);},
            CSTaskGroupIndex::FrameBegin,
        );
        cs_task.run_recurring
        (
            |data: &FD4TaskData| {(er_mod.frame_end)(data);},
            CSTaskGroupIndex::FrameEnd,
        );
    }
}
