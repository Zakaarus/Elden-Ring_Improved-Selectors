use eldenring::cs::PlayerIns;

use fromsoftware_shared::OwnedPtr;

use super::get_main_player;

pub fn change_spell(player_option:Option<&mut OwnedPtr<PlayerIns>>,slot:i32)
    -> Option<()>
{
    let player = player_option.or_else(||return get_main_player())?;
    player.player_game_data.equipment.equip_magic_data.selected_slot = slot;
    return Some(());
}

#[cfg(debug_assertions)]
use eldenring_util::singleton::get_instance;
#[cfg(debug_assertions)]
use eldenring::cs::CSFeManImp;
/// Not working. It actually breaks the UI.
#[cfg(debug_assertions)]
pub fn _show_ui()
    -> Option<()>
{
    //SAFETY: See get_instance
    unsafe 
    {
        let fe_man_imp = get_instance::<CSFeManImp>()
            .unwrap_or_else
            (|error|{
                println!("LOOKUP FAIL: {error:}");
                return None;
            })
            .or_else
            (||{
                println!("LOOKUP FAIL (NONE FOUND)");
                return None;
            })?;
        fe_man_imp.enable_hud = !fe_man_imp.enable_hud;
    }
    return Some(())
}