//! functions for dealing with byte-code

use core::fmt::Debug;
use alloc::{format, string::String, vec::Vec};
use crate::drive_train::DriveTrain;

/// A single bytecode instruction for the robot
#[derive(Clone, Copy, PartialEq, Eq)]
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
}

impl Debug for ByteCode { // change back to display if needed
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[inline]
        fn display_voltage(voltage: i32) -> String {
            if voltage.is_negative() {
                format!("{voltage:?}")
            } else {
                format!("+{voltage:?}")
            }
        }
        
        match self {
            ByteCode::Cycle(x) => write!(f, "c +{x:?}"),
            ByteCode::LeftDrive { voltage } => write!(f, "ld {}", display_voltage(*voltage)),
            ByteCode::RightDrive { voltage } => write!(f, "rd {}", display_voltage(*voltage)),
        }
    }
}

/// Executes bytecode and pops it off the bytecode vec for each tick-cycle
#[inline]
pub fn execute(bytecode: &mut Vec<ByteCode>, drive_train: &mut DriveTrain) {
    while let Some(inst) = bytecode.pop() {
        match inst {
            // skip a cycle while consuming the cycle inst
            // (must be zero)
            ByteCode::Cycle(0) => return,

            // skip a cycle without consuming the cycle inst
            ByteCode::Cycle(x) => {
                bytecode.push(ByteCode::Cycle(x-1));
                return
            },

            // updates the drive-train motors
            ByteCode::LeftDrive { voltage } => drive_train.drive_left(voltage),
            ByteCode::RightDrive { voltage } => drive_train.drive_right(voltage),
        }
    }
}

/// generates a vector of bytecode instructions from an ascii representation
#[macro_export]
macro_rules! ascii_bytecode {
    // user-facing api for the macro
    ($($inst:ident $prefix:tt $val:tt;)*) => {
        [
            $($crate::ascii_bytecode!(@internal $inst $prefix $val)),*
        ]
    };

    // instructions

    // Cycle
    (@internal c $prefix:tt $x:literal) => {
        $crate::bytecode::ByteCode::Cycle($x)
    };
    // LeftDrive
    (@internal ld $prefix:tt $x:literal) => {
        $crate::bytecode::ByteCode::LeftDrive { voltage: 0 $prefix $x }
    };
    // RightDrive
    (@internal rd $prefix:tt $x:literal) => {
        $crate::bytecode::ByteCode::RightDrive { voltage: 0 $prefix $x }
    };
}
