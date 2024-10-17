//! Functions that both determine the controls of the robot and also generate bytecode insts

use safe_vex::controller::{self, Controller};
use crate::{bytecode::ByteCode, config};

/// Get the belt bytecode inst
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

