use eldenring::fd4::FD4TaskData;

mod spell_selector;
mod action_reader;

/// For each mod put it here.
/// Import it, place its `ERMod` into the array, increase the array size.
pub const MOD_LIST:[ERMod; 2] = [action_reader::MOD,spell_selector::MOD]; 
// Maybe a macro can be made to automate the impport->insert->resize process.

/*<==========================================================================>*/

/// Store the mod's entry points here.
/// The `FrameFn`s are registered in order but run in parallel between mods.
/// `init`s are run in order.
pub struct ERMod
{
    pub context:&'static str,
    pub frame_begin:FrameFn,
    pub frame_end:FrameFn,
    pub init:fn()
}

type FrameFn = fn(&FD4TaskData);