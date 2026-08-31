use std::{num::NonZero, sync::atomic::{AtomicBool, AtomicI32, Ordering}};
use anyhow::anyhow;

use crate::attempt;

use super::base::SETTINGS;
use super::super::super::utils::{MAGICS, refresh_magic};
mod no_miscast;
use no_miscast::{notify_hand, Hand};
pub mod cast_slot;
use cast_slot::{cast_slot};
//#[cfg(debug_assertions)] use super::show_ui;

/// Temporary slot that is available usually for one frame.
/// Possibly longer, but I don't think so.
pub fn begin_slot() 
    -> Option<i32> 
{   
    MAGIC_SLOTS.temp.1.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
        //.inspect_err(|_|{println!("Begin Slot - Compare Exchange: No temp slot")})
        .ok()?;
    return Some(MAGIC_SLOTS.temp.0.load(Ordering::Relaxed));
}

/// Permanent slot that persists through frames.
/// It should be set at the end of the frame ... Probably not unconditionally. `cast_slot` may need extra frames of temp slot.
pub fn end_slot() 
    -> Option<i32> 
{
    return true//(CASTING.1.load(Ordering::Relaxed) == -1)
        .then(get_end_slot);
}
fn get_end_slot()
    -> i32
    {return MAGIC_SLOTS.persist.load(Ordering::Relaxed);}


static MAGIC_SLOTS:MagicSlots = MagicSlots
{
    persist:AtomicI32::new(0),
    temp:(AtomicI32::new(0),AtomicBool::new(false))
};

/// persist is set at the end of the frame, temp is set at the beginning of the frame then set to false.
struct MagicSlots
{
    persist:AtomicI32,
    temp:(AtomicI32,AtomicBool)
}

/* <=====================================================================================================================================> */

fn to_slot(raw_slot:i32)
{
    attempt!
    {[]("to_slot")
        //#[cfg(debug_assertions)] show_ui();
        let slot = bound_slot(raw_slot).ok_or_else(||anyhow!("Failed to get bound slot"))?;
        MAGIC_SLOTS.persist.store(slot, Ordering::Relaxed);
    }
}

fn temp_slot(raw_slot:i32)
{
    attempt!
    {[]("temp_slot")
        let slot = bound_slot(raw_slot).ok_or_else(||anyhow!("Failed to get bound slot"))?;
        MAGIC_SLOTS.temp.0.store(slot, Ordering::Relaxed);
        MAGIC_SLOTS.temp.1.store(true, Ordering::Relaxed);
    }
}

/* <=====================================================================================================================================> */

fn bound_slot(raw_slot:i32)
    -> Option<i32>
{
    if SETTINGS.auto_refresh {refresh_magic();}
    let len:i32 = NonZero::new(MAGICS.1.load(Ordering::Relaxed))?.get();
    return raw_slot.checked_rem_euclid(len);
}

/* <=====================================================================================================================================> */

/// perform action.
pub fn action(action:&str)
{
    let prefix_slot = |prefix:&str| 
        return action
            .strip_prefix(prefix)
            .ok_or_else(||return anyhow!("Not to_slot."))?
            .parse::<i32>()
            .map_err(|error|anyhow!(error));

    attempt!
    {[]("Spell Selector Action")
        match action
        {
            "notify_righthand" => notify_hand(Hand::RIGHT),
            "notify_lefthand" => notify_hand(Hand::LEFT),
            "next" => to_slot(get_end_slot().checked_add(1).ok_or_else(||return anyhow!("Next - Slot+1 failed due to Integer Overflow???"))?),
            "prev" => to_slot(get_end_slot().checked_sub(1).ok_or_else(||return anyhow!("Prev - Slot-1 failed due to Integer Overflow???"))?),
            "cast" => cast_slot(None),
            _ if let Ok(slot) = prefix_slot("to_") => to_slot(slot),
            _ if let Ok(slot) = prefix_slot("cast_") => cast_slot(Some(slot)),
            _ if let Ok(slot) = prefix_slot("temp_") => temp_slot(slot),
            _ => return Err(anyhow!("Unknown Control: {action}"))
        }
    }
}
