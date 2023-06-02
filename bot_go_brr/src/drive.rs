use vex_rt::prelude::*;

pub struct Drive {
    pub left_drive: Motor,
    pub right_drive: Motor,
}

impl Drive {
    pub fn run(&mut self, left: i8, right: i8) {
        self.left_drive.move_i8(left).unwrap();
        self.right_drive.move_i8(right).unwrap();
    }
}
