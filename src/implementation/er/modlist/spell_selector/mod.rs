use std::sync::LazyLock;

use crate::implementation::er::utils::{actions_this_frame, get_action_bindings};
use crate::settings::Config;
use super::super::utils::{get_world_chr_man, get_main_player};
use super::ERMod;

mod internal;
use internal::{spell_select, MAGIC_SLOT};

/* <=====================================================================================================================================> */

fn frame_begin(){begin_attempt();}
fn frame_end(){end_attempt();}
pub const MOD:ERMod = ERMod  
{
    context:"spell_selector",
    frame_begin,
    frame_end 
};

static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new(MOD.context));
static ACTION_BINDINGS: LazyLock<Vec<(String,Vec<String>)>> = LazyLock::new(||{return get_action_bindings(&CONFIG);});

/* <=====================================================================================================================================> */

fn begin_attempt() 
    -> Option<()>
{
    let world_chr_man = get_world_chr_man()?;
    let main_player = get_main_player(world_chr_man)?;
    let begin_magic_slot = spell_select(&actions_this_frame(&ACTION_BINDINGS))?;
    main_player.player_game_data.equipment.equip_magic_data.selected_slot = begin_magic_slot;
    return Some(());
}

fn end_attempt() 
    -> Option<()>
{
    let world_chr_man = get_world_chr_man()?;
    let main_player = get_main_player(world_chr_man)?;
    //SAFETY: todo GET RID OF END_MAGIC_SLOT UNSAFETY
    main_player.player_game_data.equipment.equip_magic_data.selected_slot = unsafe { MAGIC_SLOT };
    return Some(());
}

