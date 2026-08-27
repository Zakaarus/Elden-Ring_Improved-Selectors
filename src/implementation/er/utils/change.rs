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
        
        //SAFETY: See .as_mut
        unsafe
            {player.player_game_data.as_mut().equipment.equip_magic_data.selected_slot = slot;}
    };
}


/// Not working. It actually breaks the UI.
#[cfg(debug_assertions)]
pub fn _show_ui()
{
    use fromsoftware_shared::FromStatic;
    use eldenring::cs::{CSFeManHudState, CSFeManImp};
    //use anyhow::anyhow;
    
    attempt!
    {("Show UI Function")
        //SAFETY: See instance_mut
        unsafe 
        {
            let fe_man_imp:&mut CSFeManImp = CSFeManImp::instance_mut()?;
                //.ok_or_else(||return anyhow!("Fe Man Imp not found."))?;
            fe_man_imp.hud_state = CSFeManHudState::ShowAll;
        }
    };
}