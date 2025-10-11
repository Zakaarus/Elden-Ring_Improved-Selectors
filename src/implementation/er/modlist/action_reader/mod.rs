use std::{sync::LazyLock, thread};
use rdev::{Button, Event, EventType, listen};
use tokio::runtime::Runtime;
use win_key_event::init_custom_key_listener;
use eldenring::fd4::FD4TaskData;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

use crate::{attempt, settings::Config};

use super::ERMod;
use super::super::utils::get_world_chr_man;
mod keybinds;
pub use keybinds::register_bindings;
use keybinds::{KEYBINDS, KEYBIND_BUFFER};
mod special_codes;
use special_codes::{MIDDLE, LEFT, RIGHT, M4, M5, SCROLL_UP, SCROLL_DOWN, SCROLL_RIGHT, SCROLL_LEFT};
mod action;
use action::action;

//this implementation is so ugly because win_key_event doesn't support any mouse inputs outside of LMB/RMB, 
//and rdev doesn't detect keyboard input. So I have to use both.
//and a bunch of other hacky solutions on top of that... Eh...
//I need to clean it up.

pub const MOD: ERMod = ERMod
{
    context:"action_reader",
    frame_begin,
    frame_end,
    init
};
const fn frame_end(_data:&FD4TaskData){}
static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new(MOD.context));
fn init()
{
    thread::spawn(input_polling);
    register_bindings(&CONFIG, action);
}
fn frame_begin(_data:&FD4TaskData) 
{
    attempt!
    {["World Chr Man not found."] ("Action Reader Frame Begin")
        get_world_chr_man()?; //Once in the world, finalise the keybinds.
        KEYBINDS.get_or_init
        (||{
            let mut buffer = KEYBIND_BUFFER.lock()
                .unwrap_or_else(|error|panic!("MUTEX LOCK ERROR?!: {error:#?}"));
            let keybinds = buffer.clone();
            buffer.clear();
            drop(buffer);
            return keybinds;
        });
    };
}

/* <=====================================================================================================================================> */

#[flux_rs::trusted] //does not work properly.
fn input_polling() 
{
    let rt = Runtime::new()
        .unwrap_or_else(|error|panic!("TOKIO RUNTIME ERROR: {error:#?}"));
    rt.block_on
    (async{
        let _listener = init_custom_key_listener
        (
            Box::new(key_down_callback),
            Box::new(key_up_callback),
            (0x07_i32..0xFE_i32)
                .collect(),
            10,
        );
    });
    listen(rdev_callback)
        .unwrap_or_else(|error| panic!("RDEV LISTENER ERROR: {error:#?}"));
}

/* <=====================================================================================================================================> */

fn input(key:i32)
{
    attempt(key);
    fn attempt(key:i32)
        -> Option<()>
    {
        let keybinds = KEYBINDS.get()?;
        for keybind in keybinds.iter()
            .filter
            (|keybind| 
                return keybind.bind.contains(&key) 
                    && keybind.bind.iter()
                        .all(|&input| return is_held(input))
            )
        {
            #[cfg(debug_assertions)]println!("ACTION: {}",keybind.action);
            (keybind.callback)(keybind.action);
        }

        return Some(());
    }
}

fn key_down_callback(key:i32){input(key);}
const fn key_up_callback(_key:i32){}
#[expect(clippy::needless_pass_by_value, reason = "listen arg requires PBV parameter")]
fn rdev_callback(event:Event)
{
    match event.event_type 
    {
        EventType::ButtonPress(Button::Middle) => key_down_callback(MIDDLE),
        EventType::ButtonRelease(Button::Middle) => key_up_callback(MIDDLE),
        EventType::ButtonPress(Button::Left) => key_down_callback(LEFT),
        EventType::ButtonRelease(Button::Left) => key_up_callback(LEFT),
        EventType::ButtonPress(Button::Right) => key_down_callback(RIGHT),
        EventType::ButtonRelease(Button::Right) => key_up_callback(RIGHT),
        EventType::ButtonPress(Button::Unknown(1)) => key_down_callback(M4),
        EventType::ButtonRelease(Button::Unknown(1)) => key_up_callback(M4),
        EventType::ButtonPress(Button::Unknown(2)) => key_down_callback(M5),
        EventType::ButtonRelease(Button::Unknown(2)) => key_up_callback(M5),
        EventType::ButtonPress(Button::Unknown(i)) 
            | EventType::ButtonRelease(Button::Unknown(i)) => {println!("??? UNSUPPORTED MOUSE BUTTON {i:}",);},
        EventType::Wheel { delta_x,delta_y } => 
        {
            if delta_y > 0 {key_down_callback(SCROLL_UP);}
            if delta_y < 0 {key_down_callback(SCROLL_DOWN);}
            if delta_x > 0 {key_down_callback(SCROLL_RIGHT);}
            if delta_x < 0 {key_down_callback(SCROLL_LEFT);}
        },
        EventType::KeyPress(_) | EventType::KeyRelease(_) | EventType::MouseMove { .. }  => {}
    }
}

/* <=====================================================================================================================================> */

fn is_held(key:i32)
    -> bool
{
    if (0x07_i32..0xFE_i32).contains(&key){return key_state(key);}
    if (0x1007_i32..0x10FE_i32).contains(&key)
    {
        #[expect(clippy::arithmetic_side_effects, reason = "Preceeding check should ensure key is always larger than 0x1000")]
        return !key_state(key-0x1000);
    }
    return true;
}
fn key_state(key:i32)
    -> bool
{
    //SAFETY: ... C.
    unsafe{return GetAsyncKeyState(key) & i16::MIN != 0;}
}
