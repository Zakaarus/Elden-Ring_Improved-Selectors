//! Credits to:
//! - chainfailure for examples that showed how to use eldenring-rs's `get_instance` (as well as making the eldenring-rs crate)
//! - axd1x8a for giving a working param lookup example.
mod magic;
pub use magic::{*, MagicType::{Sorcery, Incantation, Neither, Both}};

mod weapon;
pub use weapon::*;

mod instances;
pub use instances::*;

pub fn refresh_all()
{
    refresh_magic();
    refresh_weapons();
}

