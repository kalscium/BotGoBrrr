extern crate alloc;

use crate::drive::Drive;
use core::time::Duration;
use vex_rt::{prelude::*, select};
use alloc::string::*;
use crate::{
    controller,
    drive::DriveArg,
    config::{Config, RunMode},
    algor::Algor,
    utils::*,
    button::ButtonArg,
    record::Record,
};

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

        // Init controller screen data
        let screen: &mut vex_rt::controller::Screen = &mut self.controller.screen;
        screen.print(0, 0, "tick: null");
    }

    fn autonomous(&mut self, _ctx: Context) {
        println!("autonomous");
        let mut l = Loop::new(Duration::from_millis(Config::TICK_SPEED));
        let mut tick: u32 = 0;
        loop {
            self.update_screen(&tick);

            // Autonomous Movement
            let arg: Option<DriveArg> = Algor::GAME_AUTO.get(&tick); // Update drive according to Autonomous algorithm
            if arg.is_none() { break }
            let arg: DriveArg = arg.unwrap();
            
            // Logging
            self.drive.lock().drive(arg);

            select! {
                _ = _ctx.done() => break,

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => {
                    tick += 1; // update tick
                    continue;
                },
            }
        }
    }

    fn opcontrol(&mut self, ctx: Context) {
        println!("opcontrol");
        // This loop construct makes sure the drive is updated every 100 milliseconds.
        let mut l = Loop::new(Duration::from_millis(Config::TICK_SPEED));
        let mut tick: u32 = 0;
        let mut record: Record = Record::new(DriveArg::Stall(ButtonArg::Null, false));
        loop {
            self.update_screen(&tick);

            // Movement
            let arg: DriveArg = match Config::RUN_MODE {
                RunMode::Practice => controller::Packet::new(&self.controller).gen_arg(), // Update drive according to controller packet
                RunMode::Autonomous => Algor::FULL_AUTO.get(&tick).unwrap(), // Update drive according to Autonomous algorithm
                // (Similar to practice)
                RunMode::Competition if Algor::GAME_AUTO.is_finished(&tick) => controller::Packet::new(&self.controller).gen_arg(), // If competition autonomous period finished use driver control
                RunMode::Competition => Algor::GAME_AUTO.get(&tick).unwrap(), // If autonomous period isn't finished, use autonomous control
                RunMode::Record => record.record(controller::Packet::new(&self.controller).gen_arg()), // Records new packets and logs them
            };

            // Logging
            if let RunMode::Record = Config::RUN_MODE {} // Log Drive Arg if not record mode and if wanted in config
            else if Config::LOG_DRIVE_ARG { arg.log(&tick) }

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

impl Bot {
    pub fn update_screen(&mut self, tick: &u32) {
        let screen = &mut self.controller.screen;
        screen.clear_line(0);
        screen.print(0, 0, &(String::from("tick: ") + &tick.to_string()));
    }
}