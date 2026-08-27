use std::{thread, time::Duration};
use device_query::{DeviceEvents, DeviceEventsHandler, DeviceQuery, DeviceState, Keycode, MouseButton};
use willhook::{HookBuilder, InputEvent, MouseEventType, MouseWheel, MouseWheelDirection};
// Combination device_query (best working keyboard polling) and willhook (most modular polling, allowing only mouse for scrolling to be used)

use super::{keybinds::{KeyName, KeyEvent, KeyState},input};


pub fn input_polling() 
    -> !
{
    //let device_state = DeviceState::new();
    let device_events = DeviceEventsHandler::new(Duration::from_millis(10))
        .expect("Could not initialize event loop");
    
    let _key_down_guard = device_events.on_key_down(key_down_callback);
    let _key_up_guard = device_events.on_key_up(key_up_callback);
    let _mouse_down_guard = device_events.on_mouse_down(mouse_down_callback);
    let _mouse_up_guard = device_events.on_mouse_up(mouse_up_callback);
    
    let scroll_hook = HookBuilder::new()
        .with_mouse()
        .build()
        .expect("Scroll hook failed to build?!");
    loop
    {
        if let Ok(input_event) = scroll_hook.recv()
            && let InputEvent::Mouse(mouse_event) = input_event
            && let MouseEventType::Wheel(wheel_event) = mouse_event.event
            //&& !matches!(wheel_event.wheel,MouseWheel::Unknown(_))
            && let Some(direction) = wheel_event.direction
            //&& !matches!(direction,MouseWheelDirection::Unknown(_))
        {
            #[cfg(debug_assertions)] 
            #[expect(clippy::use_debug, reason = "Debug output")]
            {println!("MOUSE EVENT: {mouse_event:#?}");}
            input
            (
                &KeyEvent
                {
                    name:KeyName::Other
                    (
                        match (wheel_event.wheel,direction)
                        {
                            (MouseWheel::Horizontal, MouseWheelDirection::Forward) => "ScrollRight",
                            (MouseWheel::Horizontal, MouseWheelDirection::Backward) => "ScrollLeft",
                            (MouseWheel::Vertical, MouseWheelDirection::Forward) => "ScrollUp",
                            (MouseWheel::Vertical, MouseWheelDirection::Backward) => "ScrollDown",
                            _ => "Unknown"
                        }.to_owned()
                    ),
                    state:KeyState::Pressed
                }
            );
        } else {thread::yield_now();}
        //thread::park();
    }
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "Specific function signature required")]
fn key_up_callback(keycode:&Keycode)
{
    input
    (
        &KeyEvent
        {
            name:KeyName::Keycode(*keycode),
            state:KeyState::Released
        }
    );
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "Specific function signature required")]
fn key_down_callback(keycode:&Keycode)
{
    input
    (
        &KeyEvent
        {
            name:KeyName::Keycode(*keycode),
            state:KeyState::Pressed
        }
    );
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "Specific function signature required")]
fn mouse_down_callback(mouse_button:&MouseButton)
{
    input
    (
        &KeyEvent
        {
            name:KeyName::Other(string_from_mouse_button(*mouse_button)),
            state:KeyState::Pressed
        }
    );
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "Specific function signature required")]
fn mouse_up_callback(mouse_button:&MouseButton)
{
    input
    (
        &KeyEvent
        {
            name:KeyName::Other(string_from_mouse_button(*mouse_button)),
            state:KeyState::Released
        }
    );
}

fn string_from_mouse_button(button:MouseButton)
    -> String
{
    return match button
    {
        1 => "m1",
        2 => "m2",
        3 => "m3",
        4 => "m4",
        5 => "m5",
        _ => "Unknown"
    }.to_owned();
}


pub fn is_held(key:&KeyName)
    -> bool
{
    let device_state = DeviceState::new();
    return match key.to_owned()
    {
        KeyName::Keycode(keycode) => device_state.get_keys().contains(&keycode),
        KeyName::Other(input_code) => match input_code.as_str()
        {
            _ if input_code.starts_with('m') 
                && input_code.len() == 2 => 
                (input_code.as_bytes().get(1).copied().unwrap_or_default() as char)
                    .to_digit(10)
                    .and_then
                    (|mx|
                        return device_state
                            .get_mouse()
                            .button_pressed
                            .get(mx as usize).copied()
                    )
                    .unwrap_or(false),
            _ => false
        },
    }
}







