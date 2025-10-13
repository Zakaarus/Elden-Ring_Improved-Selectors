use std::{sync::{LazyLock, Mutex, atomic::Ordering}, thread, time::Duration};

use anyhow::anyhow;

use crate::{attempt, implementation::{er::utils::{MagicType::{self, Both, Incantation, Neither, Sorcery}, WEAPONS, refresh_weapons}}};

use super::{SETTINGS, MAGIC_SLOTS, end_slot, temp_slot, MAGICS};

pub mod hand 
{
    pub const LEFT:i32 = 0;
    pub const RIGHT:i32 = 1;
}

pub fn notify_hand(hand:i32)
{
    attempt!
    {["Off"] ("Hand Notify")
        if !SETTINGS.no_miscast {return Err(anyhow!("Off"));}
        if SETTINGS.auto_refresh {refresh_weapons();}
        let weapons = WEAPONS.lock()
            .map_err(|error|return anyhow!("{error:#?}"))?;
        let (off,notify):(MagicType,MagicType) =
            match hand
            {
                hand::LEFT => (weapons.right.0.magic_type,weapons.left.0.magic_type),
                hand::RIGHT => (weapons.left.0.magic_type,weapons.right.0.magic_type),
                _=> return Err(anyhow!("INCORRECT SELECTED WEAPON ARGUMENT?"))
            };
        drop(weapons);

        let persist:usize = MAGIC_SLOTS.persist.load(Ordering::Relaxed)
            .try_into()?;
        
        let slot_type = MAGICS.0.lock()
            .map_err(|error|return anyhow!("{error:#?}"))?
            .get(persist)
            .ok_or_else(||return anyhow!("Bad persist index"))?
            .magic_type;
        match (notify,off,slot_type)
        {
            (Sorcery,Both|Incantation,Incantation)
                | (Incantation,Both|Sorcery,Sorcery) =>
            {
                //Mismatch. Offhand does compensate - so it is intentional.
                //Swap spell to a compatible one. 
                //This gives staff-seal the ability to have two spells selected.
                //Don't think there's a need for fallback here.
                #[cfg(debug_assertions)] println!("INTENTIONAL");
                miscast_intentional();
            }

            (Sorcery,Neither|Sorcery,Incantation) 
                | (Incantation,Neither|Incantation,Sorcery) => 
            {
                //Mismatch. Offhand does not compensate - so it is unintentional.
                //Swap weapon to a right one.
                //This ensures the spell you are casting always goes through.
                //Then fallback to previous if none of the 3 weapons work.
                #[cfg(debug_assertions)] println!("UNINTENTIONAL");
                miscast_unintentional(slot_type);
            }

            (Sorcery,_,Sorcery) 
                | (Incantation,_,Incantation)
                | (Neither|Both,_,_)
                | (_,_,Neither|Both) => 
            {
                //Match or not trying to cast a spell. Do nothing.
                #[cfg(debug_assertions)] println!("MATCH / NON SPELL");
            } 
        }
    };
}



fn refresh_split_magic()
{
    attempt! 
    {["err"] ("Split Magic Refresh")
        let new_sm = init_split_magic();
        #[cfg(debug_assertions)] println!("{new_sm:?}");
        *SPLIT_MAGIC.lock()
            .map_err(|error|return anyhow!("{error:#?}"))?
            =new_sm;
    };
}
fn init_split_magic()
    -> Box<[usize]>
{
    let magics = 
        loop 
        { 
            if let Ok(magics) = MAGICS.0.try_lock() 
                {break magics;} 
            #[cfg(debug_assertions)] println!("Init Split Magic Function: Magics Mutex lock failed."); 
            MAGICS.0.clear_poison(); 
            thread::sleep(Duration::from_secs(5)); 
        };
    let (sorceries,incantations) = magics.iter().enumerate()
        .fold
        (
            (Vec::new(),Vec::new()),
            |(mut sorceries,mut incantations),(i,magic)|{
                if matches!(magic.magic_type,Sorcery)
                    {sorceries.push(i);}
                if matches!(magic.magic_type,Incantation)
                    {incantations.push(i);}
                return (sorceries,incantations);
            }
        );
    drop(magics);

    let mut pairs = sorceries.iter().copied()
        .zip(incantations.iter().copied().cycle())
        .chain
        (
            incantations.iter().copied()
                .zip(sorceries.iter().copied().cycle())
        )
        .collect::<Box<[(usize,usize)]>>();
    pairs.sort_by_key(|&(i,_)|return i);
    return pairs.iter()
        .map(|&(_,redir)|return redir)
        .collect();
}   

static SPLIT_MAGIC:LazyLock<Mutex<Box<[usize]>>> = LazyLock::new(||{return Mutex::new(init_split_magic());});

fn miscast_intentional()
{
    attempt!
    {("Intentional Miscast")
        refresh_split_magic();
        let target_slot = *SPLIT_MAGIC.lock()
            .map_err(|error| return anyhow!("{error:#?}"))?
            .get::<usize>
            (
                end_slot()
                    .try_into()?
            )
            .ok_or_else(||return anyhow!("Invalid Split Magic Index"))?;
        temp_slot(target_slot.try_into()?);
        #[cfg(debug_assertions)] println!("{} -> {target_slot}",end_slot());
    };
}

fn miscast_unintentional(_slot_type:MagicType)
{
    attempt!
    {["unintentional unimplemented"] ("Unintentional Miscast")
        return Err(anyhow!("unintentional unimplemented"));
        #[expect(unreachable_code,reason = "Early return for testing")]
    }
    miscast_intentional();
}