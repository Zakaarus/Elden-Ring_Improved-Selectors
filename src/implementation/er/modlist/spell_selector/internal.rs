/// Read action buffer
pub fn receive_actions(actions:&[&str])
{
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
}

pub fn begin_slot() -> Option<i32> {return None}
pub fn end_slot() -> Option<i32> {return unsafe{Some(MAGIC_SLOT)}}

/// Persistent slot that persists through frames
/// It should be set at the end of the frame unconditionally
static mut MAGIC_SLOT:i32 = 1;

