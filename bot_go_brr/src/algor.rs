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
    pub fn get(&self, tick: &u32) -> Option<DriveArg> {
        let tick = *tick as usize;
        if self.0.len() <= tick { None }
        else { Some(self.0[tick].as_ref().unwrap().duplicate()) }
    }

    pub fn is_finished(&self, tick: &u32) -> bool {
        let tick = *tick as usize;
        self.0.len() <= tick
    }
}

// Algorithms
use DriveArg::*;
use ButtonArg::*;
impl Algor {
    pub const GAME_AUTO: Algor = gen_algor! {
        Forward for 20;
    };

    pub const FULL_AUTO: Algor = gen_algor! {
        Stop for 60;
        (precise) Forward(A) for 30;
        Backward for 20;
        Forward for 40;
    };
}