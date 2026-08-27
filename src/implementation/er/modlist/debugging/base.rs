
use willhook::willhook;
use std::sync::{Arc, atomic::{Ordering, AtomicBool}};

use eldenring::fd4::FD4TaskData;

use crate::implementation::er::modlist::ERMod;

pub const MOD: ERMod = ERMod
{
    context:"action_reader",
    frame_begin,
    frame_end,
    init
};

fn frame_begin(_data:&FD4TaskData)
{

}

fn frame_end(_data:&FD4TaskData)
{

}

fn init()
{
    let is_running = Arc::new(AtomicBool::new(true));
    let set_running = is_running.clone();

    let h = willhook().unwrap();

    while is_running.load(Ordering::SeqCst) {
        if let Ok(ie) = h.try_recv() {
            match ie {
                willhook::InputEvent::Keyboard(ke) => 
                {
                    println!("{:?}", ke);
                },
                willhook::InputEvent::Mouse(me) => {},//println!("{:?}", me),
                _ => println!("Input event: {:?}", ie),
            }
        } else {
            std::thread::yield_now();   
        }
    };
}