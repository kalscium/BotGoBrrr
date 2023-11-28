use safe_vex::{bot::Bot, context::Context};
use crate::drive::{Drive, DriveState, DriveArg};

pub struct Robot {
    drive: Drive,
}

impl<'a> Bot<'a> for Robot {
    #[inline]
    fn new(ctx: &'a Context) -> Self {
        Self {
            drive: Drive::new(ctx),
        }
    }

    #[inline]
    fn opcontrol(&'a mut self, context: &'a mut Context) {
        // Get the current drive-train state
        let (drive_state, flush_logs) = {
            // Get the controls from the controller safely
            let mut controller = context.controller();

            (DriveState::new(&DriveArg::new(
                controller.left_stick(),
                controller.l2(),
                controller.r2(),
                controller.up(),
                controller.down(),
            )), controller.x())
        };

        // flush logs if required
        if flush_logs {
            context.flush_logs();
        }

        self.drive.run(context, drive_state);
    }
}

safe_vex::entry!(Robot);