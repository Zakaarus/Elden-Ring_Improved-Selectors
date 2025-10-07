use eldenring::cs::PlayerIns;
use fromsoftware_shared::OwnedPtr;

use super::{get_main_player, get_world_chr_man};

pub fn change_spell(player_option:Option<&'static mut OwnedPtr<PlayerIns>>,slot:i32)
    -> Option<()>
{
    let player = 
        if player_option.is_none() {get_main_player(get_world_chr_man()?)?}
        else {player_option?};
    player.player_game_data.equipment.equip_magic_data.selected_slot = slot;
    return Some(());
}