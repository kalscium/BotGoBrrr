use crate::drive::DriveArg;
use crate::utils::log_extra;

pub struct Record {
    arg: DriveArg,
    held: u8,
}

impl Record {
    pub fn new(arg: DriveArg) -> Self {
        Self {
            arg,
            held: 0,
        }
    }

    pub fn clear(&mut self, arg: DriveArg) {
        self.arg = arg;
        self.held = 0;
    }
    
    pub fn record(&mut self, arg: DriveArg) -> DriveArg {
        if self.arg.to_string() == arg.to_string() { self.held += 1 }
        else {
            self.release();
            self.clear(arg.duplicate());
        }; arg
    }

    pub fn release(&self) {
        let arg: (&str, &str) = self.arg.to_string();
        log_extra(&(self.held as u128), "Record", arg.0, arg.1);
    }
}