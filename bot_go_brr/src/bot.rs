use safe_vex::{bot::Bot, context::Context, Mutex};
use crate::drive::{Drive, DriveState, DriveArg};

pub struct Robot<'a> {
    drive: Drive<'a>,
}

impl<'a> Bot<'a> for Robot<'a> {
    #[inline]
    fn new(ctx: &'a Mutex<Context>) -> Self {
        Self {
            drive: Drive::new(ctx),
        }
    }

    #[inline]
    fn opcontrol(&'a mut self, context: &'a Mutex<Context>) {
        // Get the controls from the controller safely
        let (left_stick, butt_l2, butt_r2, butt_up, butt_down, flush_logs) = {
            let mut context = context.lock();
            let mut controller = context.controller();
            (controller.left_stick(), controller.l2(), controller.r2(), controller.up(), controller.down(), controller.x())
        };

        // Get the current drive-train state
        let drive_state = DriveState::new(
            &DriveArg::new(left_stick, butt_l2, butt_r2, butt_up, butt_down)
        );

        if flush_logs {
            context.lock().flush_logs();
        }

        self.drive.run(drive_state);
    }
}