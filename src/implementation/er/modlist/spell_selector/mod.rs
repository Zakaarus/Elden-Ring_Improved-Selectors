use std::sync::LazyLock;

use crate::implementation::er::utils::{actions_this_frame, get_action_bindings};
use crate::settings::Config;
use super::super::utils::{get_world_chr_man, get_main_player};
use super::ERMod;

mod internal;
use eldenring::fd4::FD4TaskData;
use internal::{receive_actions, begin_slot, end_slot};

/* <=====================================================================================================================================> */

pub const MOD:ERMod = ERMod  
{
    context:"spell_selector",
    frame_begin,
    frame_end, 
    init
};

static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new(MOD.context));
static ACTION_BINDINGS: LazyLock<Vec<(String,Vec<String>)>> = LazyLock::new(||{return get_action_bindings(&CONFIG);});

/* <=====================================================================================================================================> */

const fn init(){}//nothing yet

fn frame_begin(data:&FD4TaskData) 
    -> Option<()>
{
    let world_chr_man = get_world_chr_man()?;
    let main_player = get_main_player(world_chr_man)?;
    receive_actions(&actions_this_frame(&ACTION_BINDINGS));
    main_player.player_game_data.equipment.equip_magic_data.selected_slot = begin_slot()?;
    return Some(());
}

fn frame_end(data:&FD4TaskData) 
    -> Option<()>
{
    let world_chr_man = get_world_chr_man()?;
    let main_player = get_main_player(world_chr_man)?;
    main_player.player_game_data.equipment.equip_magic_data.selected_slot = end_slot()?;
    return Some(());
}

