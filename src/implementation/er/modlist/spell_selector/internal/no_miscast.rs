use std::{num::NonZero, sync::{LazyLock, Mutex, atomic::Ordering}, thread};

use crate::implementation::er::utils::{MagicType::{self, Both, Incantation, Neither, Sorcery}, WEAPONS, refresh_weapons};

use super::{SETTINGS, MAGIC_SLOTS, bound_slot, end_slot, temp_slot, MAGICS};

pub mod hand 
{
    pub const LEFT:i32 = 0;
    pub const RIGHT:i32 = 1;
}

pub fn notify_hand(hand:i32)
    -> Option<()>
{
    if !SETTINGS.no_miscast {return None;}
    if SETTINGS.auto_refresh {refresh_weapons();}
    let weapons = WEAPONS.lock().ok()?;
    let (off,notify):(MagicType,MagicType) =
        match hand
        {
            hand::LEFT => (weapons.right.0.magic_type,weapons.left.0.magic_type),
            hand::RIGHT => (weapons.left.0.magic_type,weapons.right.0.magic_type),
            _=>panic!("INCORRECT SELECTED WEAPON ARGUMENT?")
        };
    drop(weapons);

    let persist:NonZero<usize> =
        NonZero::<usize>::new
        (
            TryInto::<usize>::try_into
            (
                bound_slot
                    (MAGIC_SLOTS.persist.load(Ordering::Relaxed))?
            )
            .ok()?
        )?;
    #[expect(clippy::indexing_slicing, reason = "persist is bounded.")]
    let slot_type = MAGICS.0.lock().ok()?[persist.get()].magic_type;
    match (notify,off,slot_type)
    {
        (Sorcery,Both|Incantation,Incantation)
            | (Incantation,Both|Sorcery,Sorcery) =>
        {
            //Mismatch. Offhand does compensate - so it is intentional.
            //Swap spell to a compatible one. 
            //This gives staff-seal the ability to have two spells selected.
            //Don't think there's a need for fallback here.
            miscast_intentional();
        }

        (Sorcery,Neither|Sorcery,Incantation) 
            | (Incantation,Neither|Incantation,Sorcery) => 
        {
            //Mismatch. Offhand does not compensate - so it is unintentional.
            //Swap weapon to a right one.
            //This ensures the spell you are casting always goes through.
            //Then fallback to previous if none of the 3 weapons work.
            miscast_unintentional(slot_type)
                .or_else(miscast_intentional);
        }

        (Sorcery,_,Sorcery) 
            | (Incantation,_,Incantation)
            | (Neither|Both,_,_)
            | (_,_,Neither|Both) => {} //Match or not trying to cast a spell. Do nothing.
    }
    return Some(());
}

fn refresh_split_magic()
    -> Option<()>
{
    let new_sm = init_split_magic();
    *SPLIT_MAGIC.lock().ok()?=new_sm;
    return Some(());
}
fn init_split_magic()
    -> Vec<usize>
{
    let magics = 
        loop 
        { 
            if let Ok(magics) = MAGICS.0.lock() 
                {break magics;} 
            #[cfg(debug_assertions)]println!("RETRYING NAGICS"); 
            MAGICS.0.clear_poison(); 
            thread::yield_now(); 
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
        .collect::<Vec<(usize,usize)>>();
    pairs.sort_by_key(|&(i,_)|return i);
    return pairs.iter()
        .map(|&(_,redir)|return redir)
        .collect();
}   

static SPLIT_MAGIC:LazyLock<Mutex<Vec<usize>>> = LazyLock::new(||{return Mutex::new(init_split_magic());});

fn miscast_intentional()
    -> Option<()>
{
    #[cfg(debug_assertions)]println!("INTENTIONAL");
    refresh_split_magic();
    let target_slot = *SPLIT_MAGIC.lock().ok()?
        .get::<usize>
        (
            end_slot()
                .try_into().ok()?
        )?;
    temp_slot(target_slot.try_into().ok()?);
    #[cfg(debug_assertions)]
    println!("{} -> {target_slot}",end_slot());
    return Some(());
}

const fn miscast_unintentional(_slot_type:MagicType)
    -> Option<()>
{
    //#[cfg(debug_assertions)]println!("UNINTENTIONAL");
    return None;
    //return Some(());
}