use eldenring::cs::{PlayerIns, WorldChrMan};
use eldenring_util::singleton::get_instance;
use fromsoftware_shared::OwnedPtr;

pub fn get_world_chr_man() 
    -> Option<&'static mut WorldChrMan>
{
    //SAFETY: See get_instance
    return unsafe { get_instance::<WorldChrMan>() }
        .unwrap_or_else
        (|_error|{
            #[cfg(debug_assertions)]println!("WORLD_CHR_MAN ERROR: {_error:#?}");
            return None;
        });
}

pub const fn get_main_player(world_chr_man:&'static mut WorldChrMan) 
    -> Option<&'static mut OwnedPtr<PlayerIns>>
{
    return world_chr_man.main_player
        .as_mut();
}

pub fn equipped_magic()
    -> Vec<i32>
{
    return get_world_chr_man()
        .and_then
        (|world_chr_man|
            return get_main_player(world_chr_man)
        )
        .map
        (|player|
            return player.player_game_data.equipment.equip_magic_data.entries
                .iter()
                .filter_map
                (|entry| 
                    return (!(entry.charges==0 || entry.param_id==-1))
                        .then_some(entry.param_id)
                )
                .collect::<Vec<i32>>()
        )
        .unwrap_or_default();
}