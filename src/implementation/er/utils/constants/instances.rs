#![expect(clippy::use_debug, reason = "Lots of unsafe things are happening here. If something fails, it'd be ideal to get something in console.")]
#![allow(unfulfilled_lint_expectations, reason = "Or not, idk.")]

use anyhow::{Result, anyhow};
use eldenring::{cs::{PlayerIns, WorldChrMan}, fd4::FD4ParamRepository};
use eldenring_util::singleton::get_instance;
use fromsoftware_shared::OwnedPtr;

pub fn get_world_chr_man() //I've tried using a type arg but it claims to fail the invariant no matter how I annotate it.
    -> Result<&'static mut WorldChrMan>
{
    //SAFETY: See get_instance
    return unsafe { get_instance::<WorldChrMan>()}
        .map_err(|error|return anyhow!(error))?
        .ok_or_else(||return anyhow!("World Chr Man not found."));
}
pub fn get_main_player() 
    -> Result<&'static mut OwnedPtr<PlayerIns>>
{
    return get_world_chr_man()?
        .main_player
        .as_mut()
        .ok_or_else(||return anyhow!("Main player not found."));
}

pub fn get_fd4pr() //I've tried using a type arg but it claims to fail the invariant no matter how I annotate it.
    -> Result<&'static mut FD4ParamRepository>
{
    //SAFETY: See get_instance
    return unsafe { get_instance::<FD4ParamRepository>()}
        .map_err(|error|return anyhow!(error))?
        .ok_or_else(||return anyhow!("FD4 Param Repository not found.")); 
}