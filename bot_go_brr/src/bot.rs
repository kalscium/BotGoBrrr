extern crate alloc;

use crate::drive::Drive;
use core::time::Duration;
use vex_rt::{prelude::*, select};
use alloc::string::*;
use alloc::fmt::format;
use crate::{
    controller,
    drive::DriveArg,
    config::{Config, RunMode},
    algor::Algor,
    utils::*,
    button::ButtonArg,
    record::Record,
    smooth::Smooth,
};

macro_rules! format {
    ($stuff:tt) => {
        format(format_args!($stuff))
    };
}

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
        self.log("Program not startin' yet :I...");
    }

    fn autonomous(&mut self, _ctx: Context) {
        println!("autonomous");
        self.log("Autonomous block run");
        let mut l = Loop::new(Duration::from_millis(Config::TICK_SPEED));
        let mut tick: u32 = 0;
        let algor =
            if let crate::config::RunMode::Autonomous = Config::RUN_MODE { &Algor::FULL_AUTO }
            else { &Algor::GAME_AUTO };
        loop {
            self.log_tick(&(tick as u128), "Autonomous looping...");
            // Autonomous Movement
            let arg: Option<DriveArg> = algor.get(&tick); // Update drive according to Autonomous algorithm
            if arg.is_none() {
                self.log_tick(&(tick as u128), "End of autonomous");
                break;
            }
            let arg: DriveArg = arg.unwrap();
            
            self.drive.lock().drive(arg);

            select! {
                _ = _ctx.done() => {
                    self.log_tick(&(tick as u128), "End of autonomous");
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
        println!("opcontrol");
        self.log("Opcontrol block run");
        // This loop construct makes sure the drive is updated every 100 milliseconds.
        let mut l = Loop::new(Duration::from_millis(Config::TICK_SPEED));
        let mut tick: u32 = 0;
        let mut smooth = Smooth::new();
        let mut record = Record::new(DriveArg::Stall(ButtonArg::Null, false));
        loop {
            // Movement
            let arg: DriveArg = match Config::RUN_MODE {
                RunMode::Practice => self.driver(&mut smooth), // Update drive according to controller packet
                RunMode::Autonomous => Algor::FULL_AUTO.get(&tick).unwrap(), // Update drive according to Autonomous algorithm
                // (Similar to practice)
                RunMode::Competition if Algor::GAME_AUTO.is_finished(&tick) => self.driver(&mut smooth), // If competition autonomous period finished use driver control
                RunMode::Competition => Algor::GAME_AUTO.get(&tick).unwrap(), // If autonomous period isn't finished, use autonomous control
                RunMode::Record => record.record(self.driver(&mut smooth)), // Records new packets and logs them
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
        self.log("Robot is disabled :I");
    }
}

impl Bot {
    pub fn driver(&mut self, smooth: &mut Smooth) -> DriveArg{
        let packet = controller::Packet::new(&self.controller);
        if let controller::Packet::Disconnected = packet {
            println!("Controller is not workin' :C");
            self.log("ONO Controller is broken!:C");
            DriveArg::Forward(ButtonArg::Null, false)
        } else {
            self.log("Robo workin' and driving :D");
            packet.gen_arg(smooth)
        }
    }
    
    pub fn log_tick(&mut self, tick: &u128, msg: &str) {
        // self.log(&format!("[t{tick}] {msg}"))
    }

    pub fn log(&mut self, msg: &str) {
        // self.controller.screen.clear_line(0);
        // self.controller.screen.print(0, 0, msg);
    }
}