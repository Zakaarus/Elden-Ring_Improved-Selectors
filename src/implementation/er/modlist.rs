mod spell_selector;

/// For each mod put it here.
/// Import it, append its `ERMod` to the array, increase the array size.
pub const MOD_LIST:[ERMod; 1] = [spell_selector::MOD];

/*<==========================================================================>*/
pub struct ERMod
{
    pub context:&'static str,
    pub frame_begin:fn(),
    pub frame_end:fn()
}