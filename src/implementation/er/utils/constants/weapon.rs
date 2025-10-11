use std::{num::NonZero, sync::{LazyLock, Mutex}, thread};

use anyhow::{Result, anyhow};
use eldenring::{cs::PlayerIns, param::EQUIP_PARAM_WEAPON_ST};
use fromsoftware_shared::OwnedPtr;
use crate::{attempt, implementation::handle_error};

use super::{get_fd4pr,MagicType::{self,Both, Sorcery, Incantation, Neither},get_main_player};

pub fn weapon_lookup(raw_id: i32)
    -> Option<Weapon>
{
    let nz_u_id:u32 = handle_error
    (
        bound_id(raw_id),
        &[]
    )?;
    
    //#[expect(clippy::arithmetic_side_effects, reason = "nz_u_id is non-zero and unsigned (positive)")]
    let id:u32 = nz_u_id
        .checked_sub(nz_u_id % 10_000_u32)?; //The lookup fails when the last four digits aren't turned into zeroes for some reason.

    let fd4pr = get_fd4pr();
    let param:&EQUIP_PARAM_WEAPON_ST = handle_error
    (
        fd4pr,
        &[]
    )?
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
    
    fn bound_id(raw_id:i32)
        -> Result<u32>
    {
        return Ok
        (
            NonZero::new(raw_id)
            .ok_or_else(||return anyhow!("Passed ID is 0?"))?
            .get()
            .try_into()?
        );
    }
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
            let player_attempt = get_main_player();
            match player_attempt
            {
                Ok(player) =>
                    {return player;}
                Err(error) =>
                    {handle_error::<()>(Err(error), &[]);}
            }
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
{   
    attempt!
    {
        *WEAPONS.lock()
            .map_err(|_error|return anyhow!("Weapons Mutex Poisoned"))?
            =init_weapons(None);
    };
}

pub struct EquippedWeapons
{
    pub left:(Weapon,u32),
    pub right:(Weapon,u32)
}

pub static WEAPONS:LazyLock<Mutex<EquippedWeapons>> = LazyLock::new(||{return Mutex::new(init_weapons(None));});