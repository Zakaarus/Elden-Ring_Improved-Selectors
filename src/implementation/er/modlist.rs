use eldenring::fd4::FD4TaskData;

mod spell_selector;

/// For each mod put it here.
/// Import it, append its `ERMod` to the array, increase the array size.
pub const MOD_LIST:[ERMod; 1] = [spell_selector::MOD];

/*<==========================================================================>*/
pub struct ERMod
{
    pub context:&'static str,
    pub frame_begin:FrameFn,
    pub frame_end:FrameFn,
    pub init:fn()
}

type FrameFn = fn(&FD4TaskData) -> Option<()>;