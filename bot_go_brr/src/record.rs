extern crate alloc;

use safe_vex::{pile::Pile, vex_rt::io::{println, print}, colour_format};
use crate::drive::DriveState;

pub struct Record {
    l1: Pile<i32>,
    l2: Pile<i32>,
    r1: Pile<i32>,
    r2: Pile<i32>,
    arm: Pile<i32>,
}

impl Record {
    #[inline]
    pub fn new() -> Self {
        Self {
            l1: Pile::new(),
            l2: Pile::new(),
            r1: Pile::new(),
            r2: Pile::new(),
            arm: Pile::new(),
        }
    }

    #[inline]
    pub fn record(&mut self, drive_state: DriveState) {
        self.l1.push(drive_state.l1);
        self.l2.push(drive_state.l2);
        self.r1.push(drive_state.r1);
        self.r2.push(drive_state.r2);
        self.arm.push(drive_state.arm);
    }

    #[inline]
    pub fn flush(&mut self) {
        println!("{}", colour_format![blue("\n==="), cyan(" Recorded Autonomous "), blue("===")]);
        println!("Auto::new(");

        #[inline]
        fn flush(value: &mut Pile<i32>) {
            print!("    &[");
            value.flush(|x, i| print!("({x}, {i}), "));
            println!("], ");
        }

        flush(&mut self.l1);
        flush(&mut self.l2);
        flush(&mut self.r1);
        flush(&mut self.r2);
        flush(&mut self.arm);

        println!(");")
    }
}

impl Default for Record {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}