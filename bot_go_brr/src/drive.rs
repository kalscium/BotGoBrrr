use safe_vex::prelude::*;

pub struct Drive<'a> {
    pub left_motor1: Motor<'a>,
    pub right_motor1: Motor<'a>,
    
    pub left_motor2: Motor<'a>,
    pub right_motor2: Motor<'a>,
}

impl<'a> Drive<'a> {
    #[inline]
    pub fn new(context: &'a Mutex<Context>) -> Self {
        Self {
            left_motor1: build_motor(context, 1, false),
            right_motor1: build_motor(context, 2, false),

            left_motor2: build_motor(context, 3, false),
            right_motor2: build_motor(context, 4, false),
        }
    }

    #[inline]
    pub fn run(&mut self, left: i8, right: i8) {
        self.left_motor1.bind(|x| x.move_i8(left));
        self.right_motor1.bind(|x| x.move_i8(right));

        self.left_motor2.bind(|x| x.move_i8(left));
        self.right_motor2.bind(|x| x.move_i8(right));
    }
}

#[inline]
fn build_motor(context: &Mutex<Context>, port: u8, reverse: bool) -> Motor {
    Motor::build_motor(
        context,
        port,
        safe_vex::vex_rt::motor::Gearset::ThirtySixToOne,
        safe_vex::vex_rt::motor::EncoderUnits::Rotations,
        reverse
    )
}