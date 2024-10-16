//! Rboot Bytecode Instructions

use safe_vex::{adi, motor};
use crate::config;

/// A single bytecode instruction
#[derive(Debug, Clone)]
pub enum ByteCode {
    /// Progress a specified amount of tick-cycles
    Cycle(u32),
    /// Updates the drive-train's 'goal' position
    Drive {
        /// The `x` value
        x: i32,
        /// The `y` value
        y: i32,
    },
    /// Updates the voltage of the conveyor-belt motor
    Belt(i32),
    /// Determines if the solenoid is active or not
    Solenoid(bool),
}

pub fn execute(inst: ByteCode) {
    match inst {
        ByteCode::Solenoid(val) => unsafe {
            adi::digital_write(config::solenoid::PORT, val)
                .expect("solenoid should've been configured long before this point");
        },
        ByteCode::Belt(voltage) => {
            // ignore any errors in case a motor is disconnected
            let _ = motor::move_voltage(config::motors::BELT.port, config::motors::BELT.reverse, voltage);
        },
        _ => todo!(),
    };
}
