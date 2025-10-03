use std::time::Duration;
use eldenring::{cs::{CSTaskGroupIndex, CSTaskImp}, fd4::FD4TaskData};
use eldenring_util::{program::Program, singleton::get_instance, system::wait_for_system_init, task::CSTaskImpExt};
use super::modlist::MOD_LIST;

///This is what runs when `DllMain` makes its thread
pub fn entry_point() 
{
    wait_for_system_init(&Program::current(), Duration::MAX)
        .unwrap_or_else(|error|panic!("SYSTEM INIT WAIT ERROR: {error}"));


    for er_mod in MOD_LIST {(er_mod.init)();}

    
    //SAFETY: See get_instance
    let cs_task = unsafe 
    { 
        get_instance::<CSTaskImp>()
            .unwrap_or_else(|error|panic!("FAILED SINGLETON LOOKUP ERROR: {error}"))
            .expect("CSTASKIMP RETURNED NONE?!") 
    };
    cs_task.run_recurring
    (
        move|data: &FD4TaskData| {frame_begin(data);},
        CSTaskGroupIndex::FrameBegin,
    );
    cs_task.run_recurring
    (
        move|data: &FD4TaskData| {frame_end(data);},
        CSTaskGroupIndex::FrameEnd,
    );
}

fn frame_begin(data:&FD4TaskData)
{
    for er_mod in MOD_LIST {(er_mod.frame_begin)(data);}
}

fn frame_end(data:&FD4TaskData)
{
    for er_mod in MOD_LIST {(er_mod.frame_end)(data);}
}