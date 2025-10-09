use std::{num::NonZero, sync::{LazyLock, Mutex}, thread};

use eldenring::{cs::PlayerIns, param::EQUIP_PARAM_WEAPON_ST};
use fromsoftware_shared::OwnedPtr;
use super::{get_fd4pr,MagicType::{self,Both, Sorcery, Incantation, Neither},get_main_player};


pub fn weapon_lookup(raw_id: i32)
    -> Option<Weapon>
{
    let nz_u_id:NonZero<u32> = NonZero::new(raw_id.try_into().ok()?)?;
    #[expect(clippy::arithmetic_side_effects, reason = "nz_u_id is non-zero and unsigned (positive)")]
    let id:u32 = nz_u_id.get() - (nz_u_id.get() % 10_000_u32); //The lookup fails when the last four digits aren't turned into zeroes for some reason.
    let param:&EQUIP_PARAM_WEAPON_ST = get_fd4pr()?
        .get(id)?;
    /*
    println!("magic: {:?}",param.enable_magic());
    println!("miracle: {:?}",param.enable_miracle());
    println!("sorcery: {:?}",param.enable_sorcery());
    println!("vow: {:?}",param.enable_vow_magic());
    println!("weapon_catalyst: {:?}",param.is_weapon_catalyst());
    println!("category: {:?}",param.weapon_category());
    println!("type: {:?}",param.wep_type());
    */

    let magic_type=
        match(param.enable_magic(),param.enable_miracle())
        {
            (1,1) => Both,
            (1,0) => Sorcery,
            (0,1) => Incantation,
            _ => Neither,
        };

    return Some
    (
        Weapon 
        { 
            //weapon_type:0
            magic_type
        }
    );
}

pub struct Weapon
{
    //pub weapon_type:u32
    pub magic_type:MagicType
}

fn init_weapons(player_option:Option<&mut OwnedPtr<PlayerIns>>)
    -> EquippedWeapons
{
    let player = player_option.unwrap_or_else
    (||{
        loop 
        {
            if let Some(player) = get_main_player() 
                {return player;}
            #[cfg(debug_assertions)]println!("init_weapons: RETRYING PLAYER");
            std::thread::yield_now();
        }
    });
    let equips = &player.chr_asm.equipment.selected_slots;
    let param_ids = player.chr_asm.equipment_param_ids;
    let left = 
    (
        weapon_lookup(param_ids[(equips.left_weapon_slot as usize)*2])
            .expect("LEFT HAND WEAPON MISSING?"),
        equips.left_weapon_slot
    );
    let right = 
    (
        weapon_lookup(param_ids[1+((equips.right_weapon_slot as usize)*2)])
            .expect("RIGHT HAND WEAPON MISSING?"),
        equips.left_weapon_slot
    );
    /*equips.left_weapon_slot;
    equips.right_weapon_slot;
    equips.left_arrow_slot;
    equips.right_arrow_slot;
    equips.left_bolt_slot;
    equips.right_bolt_slot;*/
    return EquippedWeapons 
    {
        left,
        right
    };
}

pub fn refresh_weapons()
    -> Option<()>
{
    *WEAPONS.try_lock().ok()?=init_weapons(None);
    return Some(());
}

pub struct EquippedWeapons
{
    pub left:(Weapon,u32),
    pub right:(Weapon,u32)
}

pub static WEAPONS:LazyLock<Mutex<EquippedWeapons>> = LazyLock::new(||{return Mutex::new(init_weapons(None));});