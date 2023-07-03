use super::drive::DriveArg;
use super::button::ButtonArg;

pub struct Item {
    data: DriveArg,
    before: Option<*const Item>,
}

pub struct Algor {
    item: Item,
}

// Algorithms
impl Algor {
    pub const AUTONOMOUS: Algor = Algor::new();
}

impl Algor {
    pub const fn item(&self) -> &Item { &self.item }

    pub const fn new() -> Self {
        Self {
            item: Item {
                data: DriveArg::Stall(ButtonArg::Null),
                before: None,
            },
        }
    }

    pub const fn add(self, arg: DriveArg) -> Self {
        Self {
            item: Item {
                data: arg,
                before: Some(&self.item as *const Item),
            }
        }
    }

    pub const fn repeat(self, arg: DriveArg, ticks: u8) -> Self {
        let mut algor = self;
        let mut i: u8 = 0;
        while i <= ticks {
            algor = algor.add(arg.duplicate());
            i += 1;
        };
        algor
    }

    pub const fn stop(self, arg: ButtonArg, ticks: u8) -> Self { self.repeat(DriveArg::Stop(arg), ticks) }
    pub const fn stall(self, arg: ButtonArg, ticks: u8) -> Self { self.repeat(DriveArg::Stall(arg), ticks) }

    pub fn get(this: &Self, tick: u128) -> DriveArg {
        let mut item: &Item = this.item();
        let mut i: u128 = 0;
        while i < tick {
            if item.before.is_none() { return DriveArg::Stop(ButtonArg::Null) }
            item = unsafe { &*item.before.unwrap() };
            i += 1;
        };

        item.data.duplicate()
    }
}
