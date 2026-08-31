mod spell_selector;
mod action_reader;
//mod debugging;

/// For each mod put it here.
/// Import it, place its `ERMod` into the array, increase the array size.
pub const MOD_LIST:[GameMod; 2] = 
[
    //debugging::MOD,
    action_reader::MOD,
    spell_selector::MOD
]; 
// Maybe a macro can be made to automate the import->insert->resize process.

/*<==========================================================================>*/

/// Store the mod's entry points here.
/// `init`s are run in order.
pub struct GameMod
{
    pub context:&'static str,
    pub init:fn()
}
