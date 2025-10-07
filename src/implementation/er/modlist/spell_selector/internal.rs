use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use super::equipped_magic;

/// Read action buffer
#[expect(clippy::arithmetic_side_effects, reason = "to_slot and temp_slot are bounded before side effects.")]
#[flux_rs::trusted] //No string comparison compatibility.
pub fn action(action:&str)
{
    match action
    {
        "notify_mainhand" => {temp_slot(0);},
        "notify_offhand" => {temp_slot(7);},
        "next" => {to_slot(end_slot()+1);},
        "prev" => {to_slot(end_slot()-1);},
        _ => if let Some(slot) = action.strip_prefix("to_")
            .and_then(|slot|return slot.parse::<i32>().ok()) 
            {to_slot(slot);}
    }
}

pub fn begin_slot() 
    -> Option<i32> 
{   
    if TEMP_SLOT.1.compare_exchange_weak(true, false, Ordering::Relaxed, Ordering::Relaxed).is_ok()
        {return Some(TEMP_SLOT.0.load(Ordering::Relaxed));}
    return None;
}
pub fn end_slot() -> i32 {return MAGIC_SLOT.load(Ordering::Relaxed);}

/// Persistent slot that persists through frames.
/// It should be set at the end of the frame unconditionally.
static MAGIC_SLOT:AtomicI32 = AtomicI32::new(0);
/// Temporary slot that is available usually for one frame.
/// Possibly longer, but I don't think so.
static TEMP_SLOT:(AtomicI32,AtomicBool) = (AtomicI32::new(0),AtomicBool::new(false));


fn to_slot(raw_slot:i32)
    ->Option<()>
{
    let slot = bound_slot(raw_slot)?;
    MAGIC_SLOT.store(slot, Ordering::Relaxed);
    return Some(());
}

fn temp_slot(raw_slot:i32)
    ->Option<()>
{
    let slot = bound_slot(raw_slot)?;
    TEMP_SLOT.0.store(slot, Ordering::Relaxed);
    TEMP_SLOT.1.store(true, Ordering::Relaxed);
    return Some(());
}


#[expect(clippy::arithmetic_side_effects, reason = "Early return if len is under 0, stopping negative indexing.")]
#[expect(clippy::modulo_arithmetic, reason = "Early return if len is under 0, stopping divide by 0.")]
fn bound_slot(raw_slot:i32)
    -> Option<i32>
{
    let magics = equipped_magic();
    let len:i32 = magics.len().try_into().ok()?;
    if len == 0_i32 {return None;}
    return Some
    (
        if raw_slot < 0 {len-1}
        else {raw_slot%len}
    );
}