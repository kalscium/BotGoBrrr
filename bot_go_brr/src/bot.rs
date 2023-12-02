use safe_vex::{bot::Bot, context::Context, Mutex};
use crate::{drive::{Drive, DriveState, DriveArg}, auto::Auto, config::Config};
#[cfg(debug_assertions)]
use crate::record::Record;

pub struct Robot {
    drive: Mutex<Drive>,
    autonomous: Mutex<Auto>,
    /// **Used buttons:** `a` & `x`
    button_before: Mutex<(bool, bool)>,
    #[cfg(debug_assertions)]
    record: Mutex<Record>,
}

impl<'a> Bot<'a> for Robot {
    #[inline]
    fn new(ctx: &'a Context) -> Self {
        Self {
            drive: Mutex::new(Drive::new(ctx)),
            autonomous: Mutex::new(Config::AUTO_COMPETITION),
            button_before: Mutex::new((false, false)),
            #[cfg(debug_assertions)]
            record: Mutex::new(Record::new()),
        }
    }

    #[inline]
    fn autonomous(&'a self, context: &'a mut Context) {
        // Get the autonomous argument
        if let Some(x) = self.autonomous.lock().next() {
            self.drive.lock().run(context, x);
        }
    }

    #[inline]
    fn opcontrol(&'a self, context: &'a mut Context) {
        // Get the current drive-train state
        let (drive_state, flush_logs) = {
            // Get the controls from the controller safely
            let mut controller = context.controller();

            // flush the recorded autonomous if required
            #[cfg(debug_assertions)]
            if controller.a() && !self.button_before.lock().0 {
                self.button_before.lock().0 = true;
                self.record.lock().flush();
            } else { self.button_before.lock().1 = false };

            (DriveState::new(&DriveArg::new(
                controller.left_stick(),
                controller.right_stick(),
                controller.l2(),
                controller.r2(),
                controller.up() || controller.x(),
                controller.down() || controller.b(),
            )), controller.y())
        };

        // flush logs if required
        if flush_logs && !self.button_before.lock().1 {
            self.button_before.lock().1 = true;
            context.flush_logs();
        } else { self.button_before.lock().1 = false };

        // Record the drive-state if not release
        #[cfg(debug_assertions)]
        self.record.lock().record(drive_state);

        self.drive.lock().run(context, drive_state);
    }
}