use eldenring::cs::PlayerIns;
use fromsoftware_shared::OwnedPtr;

use super::get_main_player;
use crate::attempt;

pub fn change_spell(player_option:Option<&mut OwnedPtr<PlayerIns>>,slot:i32)
{
    attempt!
    {("Change Spell Function")
        let player = 
            if let Some(player) = player_option
                {player}
            else
                {get_main_player()?};
        player.player_game_data.equipment.equip_magic_data.selected_slot = slot;
    };
}


/// Not working. It actually breaks the UI.
#[cfg(debug_assertions)]
pub fn _show_ui()
{
    use eldenring_util::singleton::get_instance;
    use eldenring::cs::CSFeManImp;
    use anyhow::anyhow;
    
    attempt!
    {("Show UI Function")
        //SAFETY: See get_instance
        unsafe 
        {
            let fe_man_imp = get_instance::<CSFeManImp>()?
                .ok_or_else(||return anyhow!("Fe Man Imp not found."))?;
            fe_man_imp.enable_hud = !fe_man_imp.enable_hud;
        }
    };
}