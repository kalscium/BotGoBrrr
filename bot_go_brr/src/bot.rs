use safe_vex::{bot::Bot, context::Context, port::PortManager, vex_rt::peripherals::Peripherals};

use crate::{controls, drive_train::{DriveInst, DriveTrain}};

/// The robot
pub struct Robot {
    /// The drive-train of the robot
    drive_train: DriveTrain,

    /// The previous drive-instruction
    last_inst: Option<DriveInst>,
    /// The previous joystick info
    last_joystick: Option<(i8, i8)>,
}

impl Bot for Robot {
    const TICK_SPEED: u64 = 50;

    #[inline]
    fn new(_: &Peripherals, port_manager: &mut PortManager) -> Self {
        Self {
            drive_train: DriveTrain::new(port_manager),

            last_inst: None,
            last_joystick: None,
        }
    }

    #[inline]
    fn opcontrol(&mut self, context: Context) -> bool {
        // get drive-inst
        let (drive_inst, joystick) = controls::gen_drive_inst(
            self.last_inst.take().unwrap_or(DriveInst { left: 0, right: 0 }),
            self.last_joystick.take().unwrap_or((0, 0)),
            &context.controller
        );

        // update motors
        self.drive_train.drive(&drive_inst);

        // update internal state
        self.last_inst = Some(drive_inst);
        self.last_joystick = Some(joystick);
        
        false
    }
}
