use alloc::boxed::Box;
use safe_vex::{maybe::Maybe, motor::Motor, port::PortManager};
use crate::config;

/// Represents the four-wheel drive-train of the robot
pub struct DriveTrain {
    /// the top-left drive-motor of the robot
    pub l1: Maybe<Motor>,
    /// the bottom-left drive-motor of the robot
    pub l2: Maybe<Motor>,
    /// the extra 6th motor
    pub l3: Maybe<Motor>,

    /// the top-right drive-motor of the robot
    pub r1: Maybe<Motor>,
    /// the bottom-right drive-motor of the robot
    pub r2: Maybe<Motor>,
    /// the extra 6th motor
    pub r3: Maybe<Motor>,
}

impl DriveTrain {
    #[inline]
    pub fn new(port_manager: &mut PortManager) -> Self {
        // reserve the motors in the port manager
        let _ = port_manager.take(config::drive::L1.port);
        let _ = port_manager.take(config::drive::L2.port);
        let _ = port_manager.take(config::drive::L3.port);
        let _ = port_manager.take(config::drive::R1.port);
        let _ = port_manager.take(config::drive::R2.port);
        let _ = port_manager.take(config::drive::R3.port);

        // construct the drive-train
        Self {
            l1: Maybe::new(Box::new(|| build_motor(config::drive::L1.port, config::drive::L1.reverse))),
            l2: Maybe::new(Box::new(|| build_motor(config::drive::L2.port, config::drive::L2.reverse))),
            l3: Maybe::new(Box::new(|| build_motor(config::drive::L2.port, config::drive::L3.reverse))),
            r1: Maybe::new(Box::new(|| build_motor(config::drive::R1.port, config::drive::R1.reverse))),
            r2: Maybe::new(Box::new(|| build_motor(config::drive::R2.port, config::drive::R2.reverse))),
            r3: Maybe::new(Box::new(|| build_motor(config::drive::R2.port, config::drive::R3.reverse))),
        }
    }

    /// Sets the voltage left motors of the drive-train
    #[inline]
    pub fn drive_left(&mut self, voltage: i32) {
        self.l1.get().map(|x| x.move_voltage(voltage));
        self.l2.get().map(|x| x.move_voltage(voltage));
    }

    /// Sets the voltage right motors of the drive-train
    #[inline]
    pub fn drive_right(&mut self, voltage: i32) {
        self.r1.get().map(|x| x.move_voltage(voltage));
        self.r2.get().map(|x| x.move_voltage(voltage));
    }
}

#[inline]
fn build_motor(port: u8, reverse: bool) -> Option<Motor> {
    // we know that there will **not** be more than one mutable reference to the motor
    unsafe { Motor::new(
        port,
        config::drive::GEAR_RATIO,
        config::drive::UNIT,
        reverse,
    ) }.ok()
}
