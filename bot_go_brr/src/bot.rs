use safe_vex::{bot::Bot, context::Context};
use crate::{drive::{Drive, DriveState, DriveArg}, auto::Auto, config::Config};
#[cfg(debug_assertions)]
use crate::record::Record;

pub struct Robot {
    drive: Drive,
    autonomous: Auto,
    /// **Used buttons:** `a` & `x`
    button_before: (bool, bool),
    #[cfg(debug_assertions)]
    record: Record,
}

impl<'a> Bot<'a> for Robot {
    #[inline]
    fn new(ctx: &'a Context) -> Self {
        Self {
            drive: Drive::new(ctx),
            autonomous: Config::AUTO_COMPETITION,
            button_before: (false, false),
            #[cfg(debug_assertions)]
            record: Record::new(),
        }
    }

    #[inline]
    fn autonomous(&'a mut self, context: &'a mut Context) {
        // Get the autonomous argument
        if let Some(x) = self.autonomous.next() {
            self.drive.run(context, x);
        }
    }

    #[inline]
    fn opcontrol(&'a mut self, context: &'a mut Context) {
        // Get the current drive-train state
        let (drive_state, flush_logs) = {
            // Get the controls from the controller safely
            let mut controller = context.controller();

            // flush the recorded autonomous if required
            #[cfg(debug_assertions)]
            if controller.a() && !self.button_before.0 {
                self.button_before.0 = true;
                self.record.flush();
            } else { self.button_before.1 = false };

            (DriveState::new(&DriveArg::new(
                controller.left_stick(),
                controller.right_stick(),
                controller.l2(),
                controller.r2(),
                controller.up(),
                controller.down(),
            )), controller.x())
        };

        // flush logs if required
        if flush_logs && !self.button_before.1 {
            self.button_before.1 = true;
            context.flush_logs();
        } else { self.button_before.1 = false };

        // Record the drive-state if not release
        #[cfg(debug_assertions)]
        self.record.record(drive_state);

        self.drive.run(context, drive_state);
    }
}