use crate::implementation::er::utils::refresh_all;

#[flux_rs::trusted] //No string comparison compatibility. That being said I should use a phf. Thanks flux!
pub fn action(action:&str)
{
    match action
    {
        "manual_refresh" => {refresh_all();},
        "debug" => {#[cfg(debug_assertions)]debug_action();},
        _ => {}
    }
}

#[cfg(debug_assertions)]
fn debug_action()
{
    use crate::implementation::er::utils::refresh_weapons;

    println!("Debugging...");
    refresh_weapons();
    println!("Debugged.");
}