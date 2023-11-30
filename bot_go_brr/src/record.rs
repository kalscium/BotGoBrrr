extern crate alloc;

use safe_vex::{pile::Pile, vex_rt::io::println};
use crate::drive::DriveState;

pub struct Record {
    l1: Pile<i32>,
    l2: Pile<i32>,
    r1: Pile<i32>,
    r2: Pile<i32>,
    arm: Pile<i32>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            l1: Pile::new(),
            l2: Pile::new(),
            r1: Pile::new(),
            r2: Pile::new(),
            arm: Pile::new(),
        }
    }

    pub fn record(&mut self, drive_state: DriveState) {
        self.l1.push(drive_state.l1);
        self.l2.push(drive_state.l2);
        self.r1.push(drive_state.r1);
        self.r2.push(drive_state.r2);
        self.arm.push(drive_state.arm);
    }

    pub fn flush(&mut self) {
        let l1 = self.l1.flush_owned();
        let l2 = self.l2.flush_owned();
        let r1 = self.r1.flush_owned();
        let r2 = self.r2.flush_owned();
        let arm = self.arm.flush_owned();

        println!("l1: {l1:?}");
        println!("l2: {l2:?}");
        println!("r1: {r1:?}");
        println!("r2: {r2:?}");
        println!("arm: {arm:?}");
    }
}