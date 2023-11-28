pub mod drive_state;
pub use drive_state::*;

use safe_vex::prelude::*;
use crate::config::{Config, MotorConfig};

pub struct Drive {
    /// Top-let motor of the robot
    pub l1: Motor,
    /// Bottom-left motor of the robot
    pub l2: Motor,

    /// Top-right motor of the robot
    pub r1: Motor,
    /// Bottom-right motor of the robot
    pub r2: Motor,

    /// The arm of the robot
    pub arm: Motor,
}

impl Drive {
    #[inline]
    pub fn new(context: &Context) -> Self {
        Self {
            l1: build_motor(context, Config::DRIVE_L1),
            l2: build_motor(context, Config::DRIVE_L2),
            r1: build_motor(context, Config::DRIVE_R1),
            r2: build_motor(context, Config::DRIVE_R2),
            arm: build_motor(context, Config::ARM),
        }
    }

    #[inline]
    pub fn run(&mut self, context: &Context, drive_state: DriveState) {
        let DriveState { l1, l2, r1, r2, arm } = drive_state;

        self.l1.move_voltage(context, l1);
        self.l2.move_voltage(context, l2);
        self.r1.move_voltage(context, r1);
        self.r2.move_voltage(context, r2);
        self.arm.move_voltage(context, arm);
    }
}

/// Builds a new drive-train motor
#[inline]
fn build_motor(context: &Context, motor_config: MotorConfig) -> Motor {
    Motor::build_motor(
        context,
        motor_config.port,
        Config::DRIVE_GEAR_RATIO,
        Config::DRIVE_UNIT,
        motor_config.reverse
    )
}