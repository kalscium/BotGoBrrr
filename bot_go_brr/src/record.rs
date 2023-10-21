extern crate alloc;

use crate::drive::DriveArg;
use alloc::string::ToString;

pub struct Record {
    arg: DriveArg,
    /// Amount of ticks `DriveArg` is held for
    ticks: u16,
}

impl Record {
    pub fn new(arg: DriveArg) -> Self {
        Self {
            arg,
            ticks: 1,
        }
    }

    pub fn clear(&mut self, arg: DriveArg) {
        self.arg = arg;
        self.ticks = 1;
    }
    
    pub fn record(&mut self, arg: DriveArg) -> DriveArg {
        if self.arg == arg { self.ticks += 1 }
        else {
            self.log();
            self.clear(arg);
        }; arg
    }

    pub fn log(&self) {
        use crate::utils::Log::*;
        let (name, button, precise) = self.arg.to_strings();
        List(
            &Wrap("[", &List(
                &Title("held"), ": ",
                &String(self.ticks.to_string()),
            ), "]"), " ",
            &List(
                &Title(name), "",
                &List(
                    &Wrap("(", &Title(button), ")"), " Precise: ",
                    &String(precise.to_string())
                ),
            ),
        ).log();
    }
}