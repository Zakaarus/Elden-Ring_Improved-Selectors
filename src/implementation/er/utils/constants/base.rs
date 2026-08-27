use super::refresh_weapons;
use super::refresh_magic;

pub fn refresh_all()
{
    refresh_magic();
    refresh_weapons();
}