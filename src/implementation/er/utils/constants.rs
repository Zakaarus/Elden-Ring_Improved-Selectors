use eldenring::cs::{PlayerIns, WorldChrMan};
use eldenring_util::singleton::get_instance;
use fromsoftware_shared::OwnedPtr;

pub fn get_world_chr_man() 
    -> Option<&'static mut WorldChrMan>
{
    //SAFETY: See get_instance
    return unsafe { get_instance::<WorldChrMan>() }.ok()?;
}

pub const fn get_main_player(world_chr_man:&'static mut WorldChrMan) 
    -> Option<&'static mut OwnedPtr<PlayerIns>>
{
    return world_chr_man.main_player
        .as_mut();
}

