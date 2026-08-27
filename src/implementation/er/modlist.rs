use eldenring::fd4::FD4TaskData;

mod spell_selector;
mod action_reader;
//mod debugging;

/// For each mod put it here.
/// Import it, place its `ERMod` into the array, increase the array size.
pub const MOD_LIST:[ERMod; 2] = 
[
    //debugging::MOD,
    action_reader::MOD,
    spell_selector::MOD
]; 
// Maybe a macro can be made to automate the import->insert->resize process.

/*<==========================================================================>*/

/// Store the mod's entry points here.
/// The `FrameFn`s are registered in order but run in parallel between mods.
/// `init`s are run in order.
pub struct ERMod
{
    pub context:&'static str,
    pub frame_begin:fn(&FD4TaskData),
    pub frame_end:fn(&FD4TaskData),
    pub init:fn()
}

//type FrameFn = for<'a> fn(&'a FD4TaskData);