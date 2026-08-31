use std::sync::LazyLock;
use std::time::Duration;
use std::{mem, thread};
use eldenring::cs::{CSTaskGroupIndex, CSTaskImp};
use eldenring::fd4::FD4TaskData;
use eldenring::util::system::wait_for_system_init;
use fromsoftware_shared::{Program, SharedTaskImpExt};

use crate::attempt;
use crate::implementation::er::utils::get_world_chr_man;
use crate::settings::Config;
use super::super::GameMod;
use super::{keybinds::{KEYBIND_BUFFER, KEYBINDS, KeyState, KeyEvent, Key}, input_polling,action::action, register_bindings, is_held};

pub const MOD: GameMod = GameMod
{
    context:"action_reader",
    init
};

fn init()
{
    wait_for_system_init(&Program::current(), Duration::MAX)
        .unwrap_or_else(|error|panic!("Entry Point - System Init Wait: {error}"));
    
    thread::spawn(input_polling);
    register_bindings(&CONFIG, action);
    
    let cs_task = CSTaskImp::wait_for_instance(Duration::from_secs(2))
        .unwrap_or_else(|error|panic!("Entry Point - CS Task Imp: {error}"));
    
    cs_task.run_recurring(frame_begin,CSTaskGroupIndex::FrameBegin);
}

fn frame_begin(_data:&FD4TaskData)
{
    attempt!
    {["Static object not found: WorldChrMan"] ("Action Reader Frame Begin")
        get_world_chr_man()?; //Once in the world, finalise the keybinds.
        KEYBINDS.get_or_init
        (||{
            let mut buffer = KEYBIND_BUFFER.lock()
                .unwrap_or_else(|error|panic!("Action Reader Frame Begin - Keybinds Mutex: {error:#?}"));
            return mem::take(&mut *buffer).into_boxed_slice();
        });
    };
}

// <================================================================================>

static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new(MOD.context));

pub fn input(event:&KeyEvent)
    -> Option<()>
{
    let key_held_check = |key: &Key|return is_held(&key.name) == (key.modifiers.key_state == KeyState::Held);

    let bind_check = |key: &Key|
        return match key.modifiers.key_state
        {
            KeyState::Released | KeyState::Pressed => 
                event.name == key.name 
                && event.state == key.modifiers.key_state,
            KeyState::NotHeld | KeyState::Held =>
            {
                key_held_check(key)
                || (
                    event.name == key.name 
                    && (event.state == KeyState::Pressed) == (key.modifiers.key_state == KeyState::Held)
                )
            }
        };

    for keybind in KEYBINDS.get()?.iter()
        .filter
        (|keybind|
            return keybind.bind.iter()
                .all(bind_check)
        )
    {
        #[cfg(debug_assertions)] 
        #[expect(clippy::use_debug, reason = "Debug output")]
        {
            println!("ACTION: {:#?}",keybind.action);
            //println!("KEYBIND: {:#?}", keybind.bind);
        }
        (keybind.callback)(&keybind.action);
    }

    return Some(());
}
