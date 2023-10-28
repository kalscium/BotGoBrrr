extern crate alloc;

use super::drive::DriveArg;
use super::button::ButtonArg;

#[macro_export]
macro_rules! gen_algor {
    ($([$arg:expr, $count:expr]),* $(,)?) => {{
        const RESULT: [Option::<DriveArg>; $($count+)* 0usize] = {
            let mut result: [Option::<DriveArg>; $($count+)* 0usize] = [Option::<DriveArg>::None; $($count+)* 0usize];
            let mut idx: usize = 0;
            $({
                let mut i: usize = 0;
                while i < $count {
                    result[idx] = Some($arg);
                    idx += 1;
                    i += 1;
                }
            };)*
            result
        };
        Algor(&RESULT)
    }};
    ($($(($keyword:ident))? $arg:ident$(($butt:ident))? for $count:expr);* $(;)?) => {{
        const RESULT: [Option::<DriveArg>; $($count+)* 0usize] = {
            let mut result: [Option::<DriveArg>; $($count+)* 0usize] = [Option::<DriveArg>::None; $($count+)* 0usize];
            let mut idx: usize = 0;
            $({
                let _button = ButtonArg::Null;
                $(let _button = $butt;)?
                let _precise = false;
                $(let _precise = match stringify!($keyword) { _ => true };)?

                let mut i: usize = 0;
                while i < $count {
                    result[idx] = Some($arg(_button, _precise));
                    idx += 1;
                    i += 1;
                }
            };)*
            result
        };
        Algor(&RESULT)
    }};
}

pub struct Algor(&'static [Option<DriveArg>]);
impl Algor {
    pub fn get(&self, tick: usize) -> Option<DriveArg> {
        match self.0.get(tick) {
            Some(Some(x)) => Some(*x),
            _ => None,
        }
    }
}

// Algorithms
use DriveArg::*;
#[allow(unused_imports)]
use ButtonArg::*;
impl Algor {
    pub const GAME_AUTO: Algor = gen_algor! {
        Stop for 2;
        (precise) Forward for 60;
        Right for 9;
        (precise) Forward for 28;
        Backward for 12;
        Left for 10;
        Backward for 26;
        Stop for 1;
    };
}