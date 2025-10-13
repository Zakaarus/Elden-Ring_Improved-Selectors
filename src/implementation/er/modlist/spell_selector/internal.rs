use std::{num::NonZero, sync::atomic::{AtomicBool, AtomicI32, Ordering}};

use crate::{attempt, implementation::handle_error};

use super::{SETTINGS, super::super::utils::{MAGICS, refresh_magic}};

mod no_miscast;
use anyhow::anyhow;
use no_miscast::{notify_hand, hand};
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
/// It should be set at the end of the frame unconditionally.
pub fn end_slot() -> i32 {return MAGIC_SLOTS.persist.load(Ordering::Relaxed);}
static MAGIC_SLOTS:MagicSlots = MagicSlots
{
    persist:AtomicI32::new(0),
    temp:(AtomicI32::new(0),AtomicBool::new(false))
};
struct MagicSlots
{
    persist:AtomicI32,
    temp:(AtomicI32,AtomicBool)
}

/* <=====================================================================================================================================> */

fn to_slot(raw_slot:i32)
{
    attempt(raw_slot);
    fn attempt(raw_slot: i32)
        -> Option<()>
    {
        //#[cfg(debug_assertions)] show_ui();
        let slot = bound_slot(raw_slot)?;
        MAGIC_SLOTS.persist.store(slot, Ordering::Relaxed);
        return Some(());
    }
}

fn temp_slot(raw_slot:i32)
{
    attempt(raw_slot);
    fn attempt(raw_slot: i32)
        -> Option<()>
    {
        let slot = bound_slot(raw_slot)?;
        MAGIC_SLOTS.temp.0.store(slot, Ordering::Relaxed);
        MAGIC_SLOTS.temp.1.store(true, Ordering::Relaxed);
        return Some(());
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

/// perform action
#[flux_rs::trusted] //No string comparison compatibility. That being said I should use a phf. Thanks flux!
pub fn action(action:&str)
{
    attempt!
    {[] ("Spell Selector Action")
        match action
        {
            "notify_righthand" => notify_hand(hand::RIGHT),
            "notify_lefthand" => notify_hand(hand::LEFT),
            "next" => {to_slot(end_slot().checked_add(1).ok_or_else(||return anyhow!("Next - Slot+1 failed?"))?);},
            "prev" => {to_slot(end_slot().checked_sub(1).ok_or_else(||return anyhow!("Prev - Slot-1 failed?"))?);},
            _ => 
            {
                if let Some(slot) = action.strip_prefix("to_")
                    .and_then
                    (|slot|
                        return slot.parse::<i32>()
                            .inspect_err
                            (|error|{
                                let _:Option<()> = handle_error
                                (
                                    Err(anyhow!(error.to_string())), 
                                    "to_slot Action", 
                                    &[]
                                );
                            })
                            .ok()
                    ) 
                    {to_slot(slot);}
                else {return Err(anyhow!("Unknown Control: {action}"));}
            }
        }
    }
}
