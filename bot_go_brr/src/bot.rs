extern crate alloc;

use crate::{drive::Drive, button::ButtonMan};
use core::time::Duration;
use vex_rt::{prelude::*, select};
use crate::{
    controller,
    drive::DriveArg,
    config::Config,
    algor::Algor,
    button::ButtonArg,
};

pub struct Bot {
    drive: Mutex<Drive>,
    butt_man: Mutex<ButtonMan>,
    controller: Controller,
}

impl Robot for Bot {
    fn new(peripherals: Peripherals) -> Self {
        // Robot init
        Self {
            drive: Mutex::new(Drive::new()),
            butt_man: Mutex::new(ButtonMan::new()),
            controller: peripherals.master_controller,
        }
    }

    fn initialize(&mut self, _ctx: Context) {
        // Do any extra initialization here.

        // Init controller screen data
    }

    fn autonomous(&mut self, _ctx: Context) {
        let mut l = Loop::new(Duration::from_millis(Config::TICK_SPEED));
        let mut tick: usize = 0;
        let algor = &Algor::GAME_AUTO;
        loop {
            // Autonomous Movement
            let arg: Option<DriveArg> = algor.get(tick); // Update drive according to Autonomous algorithm
            let arg = match arg {
                Some(x) => x,
                None => break,
            };

            self.drive.lock().run(arg, &mut self.butt_man.lock());

            select! {
                _ = _ctx.done() => {
                    break;
                },

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => {
                    tick += 1; // update tick
                    continue;
                },
            }
        }
    }

    fn opcontrol(&mut self, ctx: Context) {
        // This loop construct makes sure the drive is updated every 100 milliseconds.
        let mut l = Loop::new(Duration::from_millis(Config::TICK_SPEED));
        loop {
            // Movement
            let arg: DriveArg = self.driver(); // Update drive according to controller packet

            self.drive.lock().run(arg, &mut self.butt_man.lock());

            select! {
                // If the driver control period is done, break out of the loop.
                _ = ctx.done() => break,

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => {
                    continue;
                },
            }
        }
    }

    fn disabled(&mut self, _ctx: Context) {
        // This runs when the robot is in disabled mode.
    }
}

impl Bot {
    pub fn driver(&mut self) -> DriveArg{
        let packet = controller::Packet::new(&self.controller);
        if let controller::Packet::Disconnected = packet {
            DriveArg::Forward(ButtonArg::Null, false)
        } else {
            packet.gen_arg()
        }
    }
}