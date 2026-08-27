use std::sync::LazyLock;
use std::{mem, thread};
use eldenring::fd4::FD4TaskData;

use crate::attempt;
use crate::implementation::er::utils::get_world_chr_man;
use crate::settings::Config;
use super::super::ERMod;
use super::{keybinds::{KEYBIND_BUFFER, KEYBINDS, KeyState, KeyEvent, Key}, input_polling,action::action, register_bindings, is_held};

pub const MOD: ERMod = ERMod
{
    context:"action_reader",
    frame_begin,
    frame_end,
    init
};

fn init()
{
    thread::spawn(input_polling);
    register_bindings(&CONFIG, action);
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

const fn frame_end(_data:&FD4TaskData)
{

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
            println!("ACTION: {:}",keybind.action);
            println!("KEYBIND: {:#?}", keybind.bind);
        }
        (keybind.callback)(&keybind.action);
    }

    return Some(());
}
