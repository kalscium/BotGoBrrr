use safe_vex::{bot::Bot, context::Context, port::PortManager, vex_rt::peripherals::Peripherals};

use crate::{controls, drive_train::DriveTrain};

pub struct Robot {
    drive_train: DriveTrain,
}

impl Bot for Robot {
    const TICK_SPEED: u64 = 50;

    #[inline]
    fn new(_: &Peripherals, port_manager: &mut PortManager) -> Self {
        Self {
            drive_train: DriveTrain::new(port_manager),
        }
    }

    #[inline]
    fn opcontrol(&mut self, context: Context) -> bool {
        let drive_inst = controls::gen_drive_inst(&context.controller);
        self.drive_train.drive(drive_inst);
        
        false
    }
}
