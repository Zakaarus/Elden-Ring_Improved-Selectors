/// Action buffer -> slot for that frame.
/// Caller should use the conditionally return value at the beginning of the frame, then revert it to a more permanent value at the end.
pub fn spell_select(actions:&[&String]) 
    -> Option<i32> 
{


    return None;
}

/// Persistent slot that persists through frames
/// It is set at the end of the frame unconditionally
pub static mut MAGIC_SLOT:i32 = 1;

