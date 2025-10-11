use std::{num::NonZero, sync::atomic::{AtomicBool, AtomicI32, Ordering}};

use crate::implementation::handle_error;

use super::{SETTINGS, super::super::utils::{MAGICS, refresh_magic}};

mod no_miscast;
use anyhow::anyhow;
use no_miscast::{notify_hand, hand};
//#[cfg(debug_assertions)]use super::show_ui;

/// Temporary slot that is available usually for one frame.
/// Possibly longer, but I don't think so.
pub fn begin_slot() 
    -> Option<i32> 
{   
    MAGIC_SLOTS.temp.1.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
        //.inspect_err(|_|{println!("No temp slot")})
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
        //#[cfg(debug_assertions)]show_ui();
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

#[expect(clippy::arithmetic_side_effects, reason = "Early return if len is under 0, stopping negative indexing.")]
#[expect(clippy::modulo_arithmetic, reason = "Early return if len is under 0, stopping divide by 0.")]
fn bound_slot(raw_slot:i32)
    -> Option<i32>
{
    if SETTINGS.auto_refresh {refresh_magic();}
    let len:i32 = NonZero::new(MAGICS.1.load(Ordering::Relaxed))?.get();
    if len==0_i32 {return None;}
    return Some
    (
        if raw_slot < 0 {len-1}
        else {raw_slot%len}
    );
}

/* <=====================================================================================================================================> */

/// perform action
#[expect(clippy::arithmetic_side_effects, reason = "to_slot and temp_slot are bounded before side effects.")]
#[flux_rs::trusted] //No string comparison compatibility. That being said I should use a phf. Thanks flux!
pub fn action(action:&str)
{
    match action
    {
        "notify_righthand" => notify_hand(hand::RIGHT),
        "notify_lefthand" => notify_hand(hand::LEFT),
        "next" => {to_slot(end_slot()+1);},
        "prev" => {to_slot(end_slot()-1);},
        _ => 
        {
            if let Some(slot) = action.strip_prefix("to_")
                .and_then
                (|slot|return slot.parse::<i32>()
                    .inspect_err
                        (|error|{let _:Option<()> = handle_error(Err(anyhow!(error.to_string())), &[]);})
                    .ok()
                ) 
                {to_slot(slot);}
            else {println!("Unknown control: {action}");}
        }
    }
}
