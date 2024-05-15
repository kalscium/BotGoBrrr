use alloc::boxed::Box;
use safe_vex::{maybe::Maybe, motor::Motor, port::PortManager};
use crate::config;

/// A drive-train movement instruction
#[derive(Debug, Clone)]
pub struct DriveInst {
    // the left drive-motors of the robot
    pub left: i32,

    // the right drive-motors of the robot
    pub right: i32,
}

/// Represents the four-wheel drive-train of the robot
pub struct DriveTrain {
    // the top-left drive-motor of the robot
    pub l1: Maybe<Motor>,
    // the bottom-left drive-motor of the robot
    pub l2: Maybe<Motor>,

    // the top-right drive-motor of the robot
    pub r1: Maybe<Motor>,
    // the bottom-right drive-motor of the robot
    pub r2: Maybe<Motor>,
}

impl DriveTrain {
    #[inline]
    pub fn new(port_manager: &mut PortManager) -> Self {
        // reserve the motors in the port manager
        let _ = port_manager.take(config::drive::L1.port);
        let _ = port_manager.take(config::drive::L2.port);
        let _ = port_manager.take(config::drive::R1.port);
        let _ = port_manager.take(config::drive::R2.port);

        // construct the drive-train
        Self {
            l1: Maybe::new(Box::new(|| build_motor(config::drive::L1.port, config::drive::L1.reverse))),
            l2: Maybe::new(Box::new(|| build_motor(config::drive::L2.port, config::drive::L2.reverse))),
            r1: Maybe::new(Box::new(|| build_motor(config::drive::R1.port, config::drive::R1.reverse))),
            r2: Maybe::new(Box::new(|| build_motor(config::drive::R2.port, config::drive::R2.reverse))),
        }
    }

    /// Sets the voltage for each of the motors of the drive-train based on a drive instruction
    #[inline]
    pub fn drive(&mut self, inst: DriveInst) {
        self.l1.get().map(|x| x.move_voltage(inst.left));
        self.l2.get().map(|x| x.move_voltage(inst.left));
        self.r1.get().map(|x| x.move_voltage(inst.right));
        self.r2.get().map(|x| x.move_voltage(inst.right));
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
