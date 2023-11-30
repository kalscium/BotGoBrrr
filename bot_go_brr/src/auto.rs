use crate::drive::DriveState;

pub struct Auto {
    l1: (&'static [(i32, u8)], u16, u8),
    l2: (&'static [(i32, u8)], u16, u8),
    r1: (&'static [(i32, u8)], u16, u8),
    r2: (&'static [(i32, u8)], u16, u8),
    arm: (&'static [(i32, u8)], u16, u8),
}

impl Auto {
    pub const fn new(
        l1: &'static [(i32, u8)],
        l2: &'static [(i32, u8)],
        r1: &'static [(i32, u8)],
        r2: &'static [(i32, u8)],
        arm: &'static [(i32, u8)],
    ) -> Self {
        Self {
            l1: (l1, 0, 0),
            l2: (l2, 0, 0),
            r1: (r1, 0, 0),
            r2: (r2, 0, 0),
            arm: (arm, 0, 0),
        }
    }
}

macro_rules! iter_item {
    ($self:ident.$name:ident) => {{
        let this = $self.$name.0.get($self.$name.1 as usize)?;
        if $self.$name.2 == this.1 {
            $self.$name.2 = 0;
            $self.$name.1 += 1;
            return $self.next();
        } else {
            $self.$name.2 += 1;
        } this.0
    }}
}

impl Iterator for Auto {
    type Item = DriveState;
    fn next(&mut self) -> Option<DriveState> {
        Some(DriveState {
            l1: iter_item!(self.l1),
            l2: iter_item!(self.l2),
            r1: iter_item!(self.r1),
            r2: iter_item!(self.r2),
            arm: iter_item!(self.arm),
        })
    }
}