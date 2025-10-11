use anyhow::anyhow;

use eldenring::cs::PlayerIns;
use fromsoftware_shared::OwnedPtr;

use super::get_main_player;
use crate::attempt;

pub fn change_spell(player_option:Option<&mut OwnedPtr<PlayerIns>>,slot:i32)
{
    attempt!
    {
        let player = player_option
            .ok_or_else(||return anyhow!("(This error should be impossible)"))
            .or_else(|_|return get_main_player())?;
        player.player_game_data.equipment.equip_magic_data.selected_slot = slot;
    };
}


/// Not working. It actually breaks the UI.
#[cfg(debug_assertions)]
pub fn _show_ui()
{
    use eldenring_util::singleton::get_instance;
    use eldenring::cs::CSFeManImp;

    
    attempt!
    {
        //SAFETY: See get_instance
        unsafe 
        {
            let fe_man_imp = get_instance::<CSFeManImp>()?
                .ok_or_else(||return anyhow!("No Fe Man Imp found"))?;
            fe_man_imp.enable_hud = !fe_man_imp.enable_hud;
        }
    };
}