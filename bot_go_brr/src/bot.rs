use crate::drive::Drive;
use core::time::Duration;
use vex_rt::{prelude::*, select};
use crate::{controller, drive::DriveArg, config::Config};
use crate::utils::*;

pub struct Bot {
    drive: Mutex<Drive>,
    controller: Controller,
}

impl Robot for Bot {
    fn new(peripherals: Peripherals) -> Self {
        // True before code before errors
        list_ports([
            &peripherals.port01,
            &peripherals.port02,
            &peripherals.port03,
            &peripherals.port04,
            &peripherals.port05,
            &peripherals.port06,
            &peripherals.port07,
            &peripherals.port08,
            &peripherals.port09,
            &peripherals.port10,
            &peripherals.port11,
            &peripherals.port12,
            &peripherals.port13,
            &peripherals.port14,
            &peripherals.port15,
            &peripherals.port16,
            &peripherals.port17,
            &peripherals.port18,
            &peripherals.port19,
            &peripherals.port20,
            &peripherals.port21,
        ].iter());

        // Robot init
        Self {
            drive: Mutex::new(Drive::new()),
            controller: peripherals.master_controller,
        }
    }

    fn initialize(&mut self, _ctx: Context) {
        // Do any extra initialization here.
    }

    fn autonomous(&mut self, _ctx: Context) {
        println!("autonomous");
        // Write your autonomous routine here.
    }

    fn opcontrol(&mut self, ctx: Context) {
        println!("opcontrol");

        // This loop construct makes sure the drive is updated every 50 milliseconds.
        let mut l = Loop::new(Duration::from_millis(Config::TICK_SPEED));
        let mut tick: u128 = 0;
        loop {
            // Update drive according to controller packet
            let arg: DriveArg = controller::Packet::new(&self.controller).gen_arg();
            let strings: (&str, &str) = arg.to_string();
            log_extra(&tick, "Movement Arg", strings.0, strings.1);
            self.drive.lock().drive(arg);

            select! {
                // If the driver control period is done, break out of the loop.
                _ = ctx.done() => break,

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => {
                    tick += 1; // update tick
                    continue;
                },
            }
        }
    }

    fn disabled(&mut self, _ctx: Context) {
        println!("disabled");
        // This runs when the robot is in disabled mode.
    }
}