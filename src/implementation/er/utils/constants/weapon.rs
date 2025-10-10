use std::{num::NonZero, sync::{LazyLock, Mutex}, thread};

use eldenring::{cs::PlayerIns, param::EQUIP_PARAM_WEAPON_ST};
use fromsoftware_shared::OwnedPtr;
use super::{get_fd4pr,MagicType::{self,Both, Sorcery, Incantation, Neither},get_main_player};

pub fn weapon_lookup(raw_id: i32)
    -> Option<Weapon>
{
    let nz_u_id:NonZero<u32> = NonZero::new(raw_id.try_into().ok()?)?;
    //#[expect(clippy::arithmetic_side_effects, reason = "nz_u_id is non-zero and unsigned (positive)")]
    let id:u32 = nz_u_id.get()
        .checked_sub(nz_u_id.get() % 10_000_u32)?; //The lookup fails when the last four digits aren't turned into zeroes for some reason.
    let param:&EQUIP_PARAM_WEAPON_ST = get_fd4pr()?
        .get(id)?;

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
            magic_type
        }
    );
}

pub struct Weapon
{
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
            thread::yield_now();
        }
    });
    let equips = &player.chr_asm.equipment.selected_slots;
    let param_ids = player.chr_asm.equipment_param_ids;
    let left = 
    (
        #[expect(clippy::arithmetic_side_effects, reason = "weapon_slot should be bounded to 0-2")]
        weapon_lookup(*param_ids.get((equips.left_weapon_slot as usize)*2).expect("BAD PARAM_ID INDEX 1?"))
            .expect("LEFT HAND WEAPON MISSING?"),
        equips.left_weapon_slot
    );
    let right = 
    (
        #[expect(clippy::arithmetic_side_effects, reason = "weapon_slot should be bounded to 0-2")]
        weapon_lookup(*param_ids.get(1+((equips.right_weapon_slot as usize)*2)).expect("BAD PARAM_ID INDEX 2?"))
            .expect("RIGHT HAND WEAPON MISSING?"),
        equips.left_weapon_slot
    );

    return EquippedWeapons 
    {
        left,
        right
    };
}

pub fn refresh_weapons()
    -> Option<()>
{
    *WEAPONS.lock().ok()?=init_weapons(None);
    return Some(());
}

pub struct EquippedWeapons
{
    pub left:(Weapon,u32),
    pub right:(Weapon,u32)
}

pub static WEAPONS:LazyLock<Mutex<EquippedWeapons>> = LazyLock::new(||{return Mutex::new(init_weapons(None));});