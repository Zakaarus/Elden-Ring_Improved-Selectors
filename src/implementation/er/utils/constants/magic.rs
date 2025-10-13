use std::{num::TryFromIntError, sync::{LazyLock, Mutex, atomic::{AtomicI32, Ordering}}};
use MagicType::{Sorcery, Incantation, Neither};
use anyhow::{Result, anyhow};
use eldenring::{fd4::FD4ParamRepository, param::MAGIC_PARAM_ST};

use crate::{attempt, implementation::handle_error};

use super::{get_fd4pr, get_main_player,};

pub fn equipped_magic()
    -> Box<[Magic]>
{
    return get_main_player()
        .map
        (|player|
            return player.player_game_data.equipment.equip_magic_data.entries
                .iter()
                .filter_map
                (|entry| 
                    return magic_lookup(entry.param_id,None)
                        .inspect_err
                            (|error|{handle_error::<Magic>(Err(anyhow!(error.to_string())), "Equipped Magic Function - Magic Lookup", &["ID Can't be negative."]);})
                        .ok()
                )
                .collect::<Box<[Magic]>>()
        )
        .unwrap_or_default();
}

pub fn magic_lookup(id: i32, fd4pr_option:Option<&'static mut FD4ParamRepository>)
    -> Result<Magic>
{
    if id < 0_i32 {return Err(anyhow!("ID Can't be negative."));}
    let param:&MAGIC_PARAM_ST = fd4pr_option
        .ok_or_else(||return anyhow!("(This error should be impossible)"))
        .or_else(|_|return get_fd4pr())?
        .get(id.try_into()?)
        .ok_or_else(||return anyhow!("Magic not found."))?;
    return Ok
    (
        Magic
        {
            magic_type: 
                if param.sp_effect_category() == 3 
                    {Sorcery} 
                else if param.sp_effect_category() == 4
                    {Incantation}
                else
                    {Neither},
            cost:param.mp()
        }
    );
}

pub static MAGICS:LazyLock<(Mutex<Box<[Magic]>>,AtomicI32)> = LazyLock::new 
(||{
    let init = init_magic();
    return 
    (
        Mutex::new(init.0),
        AtomicI32::new(init.1) //length cached for performance
    );
});

pub fn refresh_magic()   
{
    attempt!
    {("Magic Refresh")
        let init = init_magic();
        *MAGICS.0.lock()
            .map_err(|error| return anyhow!("{error:#?}"))?
            =init.0;
        MAGICS.1.store(init.1, Ordering::Relaxed);
    };
}
fn init_magic()
    -> (Box<[Magic]>,i32)
{
    let magic_arr = equipped_magic();
    let len = magic_arr.len()
        .try_into()
        .inspect_err
            (|error:&TryFromIntError|{handle_error::<()>(Err(anyhow!(error.to_string())), "Init Magic Function",&[]);})
        .unwrap_or_default();
    #[cfg(debug_assertions)]
        for magic in &magic_arr
            {#[cfg(debug_assertions)] println!("{:#?}", magic.magic_type);}
    return (magic_arr,len);
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum MagicType{Sorcery,Incantation,Neither,Both}



pub struct Magic
{
    pub magic_type:MagicType,
    #[expect(dead_code, reason = "Cost will be used one day.")]
    pub cost:i16
}