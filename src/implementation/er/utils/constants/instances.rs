#![expect(clippy::use_debug, reason = "Lots of unsafe things are happening here. If something fails, it'd be ideal to get something in console.")]

use eldenring::{cs::{PlayerIns, WorldChrMan}, fd4::FD4ParamRepository};
use eldenring_util::singleton::get_instance;
use fromsoftware_shared::OwnedPtr;

pub fn get_world_chr_man() //I've tried using a type arg but it claims to fail the invariant no matter how I annotate it.
    -> Option<&'static mut WorldChrMan>
{
    //SAFETY: See get_instance
    return unsafe { get_instance::<WorldChrMan>() }
        .unwrap_or_else
        (|error|{
            println!("world_chr_man ERROR: {error:#?}");
            return None;
        });
}
pub fn get_main_player() 
    -> Option<&'static mut OwnedPtr<PlayerIns>>
{
    return get_world_chr_man()?
        .main_player
        .as_mut();
}

pub fn get_fd4pr() //I've tried using a type arg but it claims to fail the invariant no matter how I annotate it.
    -> Option<&'static mut FD4ParamRepository>
{
    //SAFETY: See get_instance
    return unsafe { get_instance::<FD4ParamRepository>() }
        .unwrap_or_else
        (|error|{
            println!("fd4pr ERROR: {error:#?}");
            return None;
        });
}