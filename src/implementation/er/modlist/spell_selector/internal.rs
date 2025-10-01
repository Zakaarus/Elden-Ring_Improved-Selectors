/// Action buffer -> slot for that frame.
/// Caller should use the conditionally return value at the beginning of the frame, then revert it to a more permanent value at the end.
pub fn spell_select(actions:&[&str]) 
    -> Option<i32> 
{
    let temp_slot: Option<i32> = None;
    for action in actions
    {
        //SAFETY: todo GET RID OF END_MAGIC_SLOT UNSAFETY
        unsafe 
        { 
            match action.to_owned()
            {
                "next" => {MAGIC_SLOT+=1; MAGIC_SLOT%=13}
                "prev" => {MAGIC_SLOT-=1; if MAGIC_SLOT<0 {MAGIC_SLOT=13;}}
                _ => 
                {
                    if let Some(slot) = action.strip_prefix("to_").and_then(|slot|return slot.parse::<i32>().ok()) 
                    {
                        MAGIC_SLOT = slot;
                    }
                }
            }

        }
    }
    return temp_slot;
}

/// Persistent slot that persists through frames
/// It is set at the end of the frame unconditionally
pub static mut MAGIC_SLOT:i32 = 1;

