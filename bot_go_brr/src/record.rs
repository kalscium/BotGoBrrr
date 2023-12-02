extern crate alloc;

use alloc::vec::Vec;
use safe_vex::{vex_rt::io::{println, print}, colour_format};
use crate::drive::DriveState;

pub struct Record {
    l1: Vec<(i32, u16)>,
    l2: Vec<(i32, u16)>,
    r1: Vec<(i32, u16)>,
    r2: Vec<(i32, u16)>,
    arm: Vec<(i32, u16)>,
}

impl Record {
    #[inline]
    pub fn new() -> Self {
        Self {
            l1: Vec::new(),
            l2: Vec::new(),
            r1: Vec::new(),
            r2: Vec::new(),
            arm: Vec::new(),
        }
    }

    #[inline]
    pub fn record(&mut self, drive_state: DriveState) {
        #[inline]
        fn push(vec: &mut Vec<(i32, u16)>, value: i32) {
            if let Some(x) = vec.last_mut() {
                if x.0 == value {
                    x.1 += 1;
                    return;
                }
            } vec.push((value, 1));
        }

        push(&mut self.l1, drive_state.l1);
        push(&mut self.l2, drive_state.l2);
        push(&mut self.r1, drive_state.r1);
        push(&mut self.r2, drive_state.r2);
        push(&mut self.arm, drive_state.arm);
    }

    #[inline]
    pub fn flush(&mut self) {
        println!("{}", colour_format![blue("\n==="), cyan(" Recorded Autonomous "), blue("===")]);
        println!("Auto::new(");

        #[inline]
        fn flush(value: &mut Vec<(i32, u16)>) {
            print!("    &[");
            value.iter().for_each(|(x, i)| print!("({x}, {i}), "));
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