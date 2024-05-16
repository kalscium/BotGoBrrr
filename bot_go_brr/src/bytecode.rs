//! functions for dealing with byte-code

use alloc::vec::Vec;
use safe_vex::{maybe::Maybe, motor::Motor};
use crate::drive_train::DriveTrain;

/// A single bytecode instruction for the robot
#[derive(Debug, Clone, Copy)]
pub enum ByteCode {
    /// Progresses a specified amount of tick-cycles
    Cycle(u32),

    /// Updates the voltage of the left motors of the drive-train
    LeftDrive {
        /// The voltage to apply to the motor
        voltage: i32,
    },

    /// Updates the voltage of the right motors of the drive-train
    RightDrive {
        /// The voltage to apply to the motor
        voltage: i32,
    },

    /// Updates the voltage of the conveyor-belt motor of the drive-train
    Belt {
        /// The voltage to apply to the motor
        voltage: i32,
    },
}

/// Executes bytecode and pops it off the bytecode vec for each tick-cycle
#[inline]
pub fn execute(bytecode: &mut Vec<ByteCode>, drive_train: &mut DriveTrain, belt: &mut Maybe<Motor>) {
    let mut current_inst = bytecode.pop();

    while let Some(inst) = current_inst {
        match inst {
            // skip a cycle without consuming the cycle inst
            ByteCode::Cycle(x) if x != 0 => {
                bytecode.push(ByteCode::Cycle(x-1));
                return
            },

            // skip a cycle while consuming the cycle inst
            // (must be zero)
            ByteCode::Cycle(_) => return,

            // updates the drive-train motors
            ByteCode::LeftDrive { voltage } => drive_train.drive_left(voltage),
            ByteCode::RightDrive { voltage } => drive_train.drive_right(voltage),
            
            // update the conveyor-belt motor
            ByteCode::Belt { voltage } => { belt.get().map(|motor| motor.move_voltage(voltage)); },
        }
        
        // update to next instruction
        current_inst = bytecode.pop();
    }
}
