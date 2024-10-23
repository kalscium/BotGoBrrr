//! Functions that both determine the controls of the robot and also generate bytecode insts

use safe_vex::controller::{self, Controller, ControllerAnalog};
use crate::{bytecode::ByteCode, config};

/// Get the belt bytecode instruction
pub fn belt() -> ByteCode {
    // grab the boolean activations of the bytecode 
    let up = controller::get_digital(Controller::Master, config::controls::BELT_UP);
    let down = controller::get_digital(Controller::Master, config::controls::BELT_DOWN);

    // if the up button is hit, then return a belt bytecode inst to make it spin upwards
    if let Ok(true) = up {
        return ByteCode::Belt(config::motors::BELT_SPEED);
    }
    
    // if the down button is hit, then return a belt bytecode inst to make it spin downwards
    if let Ok(true) = down {
        return ByteCode::Belt(-config::motors::BELT_SPEED);
    }

    // if there are no belt buttons being hit, make the belt stop
    ByteCode::Belt(0)
}
 /// Get the solenoid bytecode instruction
pub fn solenoid(tick: u32, solenoid_active: &mut bool, solenoid_tick: &mut u32) -> ByteCode {
    // if there hasn't been at least `config::solenoid::DELAY` ticks then do nothing
    if tick - *solenoid_tick < config::solenoid::DELAY {
        return ByteCode::Solenoid(*solenoid_active);
    }

    // otherwise check for the solenoid button being toggled
    let toggled = controller::get_digital(Controller::Master, config::controls::SOLENOID_TOGGLE);
    if let Ok(true) = toggled {
        // update the solenoid tick and active variables
        *solenoid_tick = tick;
        *solenoid_active = !*solenoid_active; // also makes it so that it returns the new state in the later block

        // rumble the controller if solenoid is active
        if *solenoid_active {
            let _ = controller::rumble(Controller::Master, ".");
        }
    }

    // return the current solenoid state
    ByteCode::Solenoid(*solenoid_active)
}

/// Get the drive bytecode instruction
pub fn drive() -> ByteCode {
    // get the joystick values (from -127..=127)
    let j1x = controller::get_analog(Controller::Master, ControllerAnalog::LeftX).unwrap_or_default();
    let j1y = controller::get_analog(Controller::Master, ControllerAnalog::LeftY).unwrap_or_default();
    let j2x = controller::get_analog(Controller::Master, ControllerAnalog::RightX).unwrap_or_default();
    let j2y = controller::get_analog(Controller::Master, ControllerAnalog::RightY).unwrap_or_default();

    // get the calculated voltages
    let j1xv = drive_controls::exp_daniel(j1x as f32 / 127.0);
    let j1yv = drive_controls::exp_daniel(j1y as f32 / 127.0);

    // if the second joystick is active, then derive an angle from it's x and y values
    let mut desired_angle = None;
    if j2x != 0 || j2y != 0 {
        desired_angle = Some(drive_controls::xy_to_angle(j2x as f32, j2y as f32));
    }

    // return the clamped drive inst
    ByteCode::Drive {
        x: j1xv,
        y: j1yv,
        desired_angle,
    }
}
