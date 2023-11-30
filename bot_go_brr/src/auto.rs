use crate::drive::DriveState;

pub struct Auto {
    idx: u8,
    i: u8,
    l1: &'static [(i32, u8)],
    l2: &'static [(i32, u8)],
    r1: &'static [(i32, u8)],
    r2: &'static [(i32, u8)],
    arm: &'static [(i32, u8)],
}

#[macro_export]
macro_rules! autonomous {
    ($({ l1: ($l1:literal, $l1r:literal), l2: ($l2:literal, $l2r:literal), r1: ($r1:literal, $r1r:literal), r2: ($r2:literal, $r2r:literal), arm: ($arm:literal, $armr:literal) }),* $(,)?) => {
        $crate::auto::Auto {
            idx: 0,
            i: 0,
            l1: &[$(($l1, $l1r)),*],
            l2: &[$(($l2, $l2r)),*],
            r1: &[$(($r1, $r1r)),*],
            r2: &[$(($r2, $r2r)),*],
            arm: &[$(($arm, $armr)),*],
        }
    };
}

macro_rules! iter_item {
    ($self:ident.$name:ident) => {
        if let Some(x) = $self.$name.get($self.idx as usize).map(|x| x.0) { x } else { return None }
    }
}

impl Iterator for Auto {
    type Item = DriveState;
    fn next(&mut self) -> Option<DriveState> {
        let drive_state = {
            let l1 = if let Some(x) = self.l1.get(self.idx as usize) { x } else { return None };
            if self.i == l1.1 {
                self.i = 0;
                self.idx += 1;
                return self.next();
            }

            DriveState {
                l1: iter_item!(self.l1),
                l2: iter_item!(self.l2),
                r1: iter_item!(self.r1),
                r2: iter_item!(self.r2),
                arm: iter_item!(self.arm),
            }
        };
        self.i += 1;
        Some(drive_state)
    }
}