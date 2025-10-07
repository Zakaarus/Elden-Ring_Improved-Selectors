use std::sync::LazyLock;
use eldenring::fd4::FD4TaskData;

use super::action_reader::register_bindings;
use crate::implementation::er::modlist::spell_selector::internal::action;
use crate::settings::Config;
use super::super::utils::{get_world_chr_man, get_main_player,change_spell,equipped_magic};
use super::ERMod;


mod internal;
use internal::{begin_slot, end_slot};

/* <=====================================================================================================================================> */

pub const MOD:ERMod = ERMod  
{
    context:"spell_selector",
    frame_begin,
    frame_end, 
    init
};

static CONFIG: LazyLock<Config> = LazyLock::new(||return Config::new(MOD.context));

/* <=====================================================================================================================================> */

fn init(){register_bindings(&CONFIG, action);}

fn frame_begin(_data:&FD4TaskData) 
    -> Option<()>
{
    let world_chr_man = get_world_chr_man()?;
    let main_player = get_main_player(world_chr_man)?;
    change_spell(Some(main_player),begin_slot()?);
    return Some(());
}

fn frame_end(_data:&FD4TaskData) 
    -> Option<()>
{
    let world_chr_man = get_world_chr_man()?;
    let main_player = get_main_player(world_chr_man)?;
    change_spell(Some(main_player),end_slot());
    return Some(());
}

/* <=====================================================================================================================================> */

