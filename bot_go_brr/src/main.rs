#![no_std]
#![no_main]

mod drive;

use drive::Drive;
use safe_vex::prelude::*;

struct MyRobot<'a> {
    drive: Drive<'a>,
}

impl<'a> Bot<'a> for MyRobot<'a> {
    #[inline]
    fn new(context: &'a Mutex<Context>) -> Self {
        Self {
            drive: Drive::new(context)
        }
    }

    #[inline]
    fn autonomous(&'a mut self, _: &'a Mutex<Context>) {
        // Write your autonomous code here
    }

    #[inline]
    fn opcontrol(&'a mut self, context: &'a Mutex<Context>) {
        // Get state of controller joysticks
        let (left_stick, right_stick, print_log) = {
            let mut context = context.lock();
            let mut controller = context.controller();
            (controller.left_stick().step(8), controller.right_stick().step(8), controller.x())
        };

        // Update the drivetrain / motors
        self.drive.run(left_stick.y, right_stick.y);

        // Flush log if x button pressed
        if print_log {
            context.lock().flush_logs();
        }
    }

    #[inline]
    fn disabled(&'a mut self, _: &'a Mutex<Context>) {
        // This runs when the robot is in disabled mode
    }
}