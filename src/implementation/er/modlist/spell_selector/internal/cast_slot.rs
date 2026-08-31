use std::sync::atomic::{AtomicU8, Ordering};

use crate::{attempt, implementation::er::utils::{WEAPONS, get_main_player, refresh_weapons}};

use super::{SETTINGS, MAGIC_SLOTS, MAGICS, to_slot};

use anyhow::anyhow;

/*#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Hand
{
    Left = 0,
    Right = 1,
}
const fn to_hand(code:u8)
    -> Option<Hand>
{
    return match code
    {
        0 => Some(Hand::Left),
        1 => Some(Hand::Right),
        _ => None
    }
}
const fn from_hand(hand_option:Option<Hand>)
    -> u8
{
    if let Some(hand) = hand_option 
    {
        return match hand 
        {
            Hand::Left => 0,
            Hand::Right => 1
        }
    }    
    return 2;
}*/

// I should NOT be doing magic numbers but my god do I not want to deal with wrapping mutexes when atomics exist.
// .0 is the hand, 0:left 1:right 2:not casting
// .1 is a temporary slot. -1:no slot 0..13 is the slot selected
//pub static CASTING: (AtomicU8, AtomicI32) = (AtomicU8::new(0),AtomicI32::new(-1));
pub static CASTING: AtomicU8 = AtomicU8::new(0);

pub fn cast_slot(raw_slot_option:Option<i32>)
{
    attempt!
    {["Off"]("cast_slot")
        if !SETTINGS.cast_slot {return Err(anyhow!("Off"));}
        if SETTINGS.auto_refresh {refresh_weapons();}

        if let Some(raw_slot) = raw_slot_option 
        {
            to_slot(raw_slot);
            /*temp_slot(raw_slot);
            CASTING.1.store(raw_slot, Ordering::Relaxed);*/
        }

        let weapons = WEAPONS.lock()
            .map_err(|error|return anyhow!("{error:#?}"))?;
        let right_hand_type = weapons.right.0.magic_type;
        let left_hand_type = weapons.left.0.magic_type;
        drop(weapons);
        
        let persist:usize = MAGIC_SLOTS.persist.load(Ordering::Relaxed)
            .try_into()?;
        
        let slot_type = MAGICS.0.lock()
            .map_err(|error|return anyhow!("{error:#?}"))?
            .get(persist)
            .ok_or_else(||return anyhow!("Bad persist index"))?
            .spell_type;

        let hand:u8 = 
            if left_hand_type == slot_type
                {0}
            else if right_hand_type == slot_type 
                {1}
            else 
                {2};

        CASTING.store(hand, Ordering::Relaxed);
    }
}

pub fn request_actions()
{
    attempt!
    {["no main player", "not casting"]("debug action request")
        let hand = CASTING.load(Ordering::Relaxed);
        if hand == 2 {return Err(anyhow!("not casting"))}

        let mut main_player = get_main_player(); //unfortunately needs to be binded in two statements
        let player = main_player.as_mut()
            .map_err(|_error|return anyhow!("no main player"))?;

        /*//SAFETY: See as_ref
        unsafe
        {
            if let Ok(usize_slot) = CASTING.1.load(Ordering::Relaxed).try_into() 
                && let Some(magic_entry) = player.player_game_data.as_ref().equipment.equip_magic_data.entries
                    .get::<usize>(usize_slot)
                {player.modules.magic.update_magic_id(magic_entry.param_id);}
        }*/
        
        //println!("CASTING = {casting}");
        if hand == 0
        {
            #[cfg(debug_assertions)] println!("CASTING SPELL LEFT HAND");
            player.modules.action_request.new_action_presses.set_magic_l(true); //<- works, but slow
            player.modules.action_request.new_action_presses.set_magic_l2(true);

            player.modules.action_request.queued_action_inputs.set_magic_l(true); //<- works, but only for 2nd cast and onwards
            player.modules.action_request.queued_action_inputs.set_magic_l2(true);
        } else if hand == 1
        {
            #[cfg(debug_assertions)] println!("CASTING SPELL RIGHT HAND");
            player.modules.action_request.new_action_presses.set_magic_r(true);
            player.modules.action_request.new_action_presses.set_magic_r2(true);

            player.modules.action_request.queued_action_inputs.set_magic_r(true);
            player.modules.action_request.queued_action_inputs.set_magic_r2(true);
        } else {return Err(anyhow!("not casting"));}
        CASTING.store(2, Ordering::Relaxed);
        //CASTING.1.store(-1, Ordering::Relaxed);
    }
}