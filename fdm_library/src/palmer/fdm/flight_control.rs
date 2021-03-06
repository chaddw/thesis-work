//Flight control function to be called by the flight control System

//Keyboard press handler
use device_query::{DeviceQuery, DeviceState, Keycode};

//Exit program
use std::process;

//Get data needed for the System to work
use crate::palmer::fdm::structures::KeyboardState;

pub fn flt_ctrl(keystate: &mut KeyboardState)
{
        //Set all states false before we know if they are being activated
        keystate.throttle_up = false; 
        keystate.throttle_down = false;
        keystate.aoa_up = false;
        keystate.aoa_down = false;
        keystate.bank_left = false;
        keystate.bank_right = false;
        keystate.flaps_down = false;
        keystate.zero_flaps = false;

        //Setup device query states
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();

        //Throttle
        if keys.contains(&Keycode::E)
        {
            keystate.throttle_up = true;
        }
        else if keys.contains(&Keycode::D)
        {
            keystate.throttle_down = true;
        }

        //Angle of attack
        if keys.contains(&Keycode::Down)
        {
            keystate.aoa_up = true;
        }
        else if keys.contains(&Keycode::Up)
        {
            keystate.aoa_down = true;
        }

        //Bank 
        if keys.contains(&Keycode::Left)
        {
            keystate.bank_left = true;
        }
        else if keys.contains(&Keycode::Right)
        {
            keystate.bank_right = true;
        }

        //Flaps
        if keys.contains(&Keycode::K)
        {
            keystate.flaps_down = true;
        }
        else if keys.contains(&Keycode::L)
        {
            keystate.zero_flaps = true;
        }

        //Quit program
        if keys.contains(&Keycode::Q)
        {
            process::exit(1);
        }
}