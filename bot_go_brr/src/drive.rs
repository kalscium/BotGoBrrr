extern crate alloc;

use vex_rt::motor::Motor;
use crate::config::Config;
use crate::button::{ButtonArg, ButtonMan};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriveArg {
    Forward(ButtonArg, bool),
    Backward(ButtonArg, bool),
    Left(ButtonArg, bool),
    Right(ButtonArg, bool),
    Stop(ButtonArg, bool),
    Stall(ButtonArg, bool),
}

impl DriveArg {
    pub fn execute(&self, drive: &mut Drive) {
        match self {
            DriveArg::Forward(_, precise) => drive.forwards(*precise),
            DriveArg::Backward(_, precise) => drive.backwards(*precise),
            DriveArg::Left(_, precise) => drive.left(*precise),
            DriveArg::Right(_, precise) => drive.right(*precise),
            DriveArg::Stop(_, _) => drive.stop(),
            DriveArg::Stall(_, _) => (),
        }
    }

    pub fn add(first: Self, second: Self) -> Self {
        match (first, second) {
            (x, DriveArg::Stop(_, _)) => x,
            (DriveArg::Stop(_, _), y) => y,
            (_, x) => x, // Favours second arg
            // (_, _) => DriveArg::Stall(ButtonArg::Null, false), // does nothing when both are activated
        }
    }

    pub fn get_button(&self) -> &ButtonArg {
        match self {
            DriveArg::Forward(x, _) => x,
            DriveArg::Backward(x, _) => x,
            DriveArg::Left(x, _) => x,
            DriveArg::Right(x, _) => x,
            DriveArg::Stop(x, _) => x,
            DriveArg::Stall(x, _) => x,
        }
    }
}

pub struct Drive {
    // Top to bottom, Left to right
    motor1: Motor,
    motor2: Motor,
    motor3: Motor,
    motor4: Motor,
}

impl Drive {
    pub fn new() -> Drive {
        Drive {
            motor1: Drive::build_motor(1),
            motor2: Drive::build_motor(2),
            motor3: Drive::build_motor(3),
            motor4: Drive::build_motor(4),
        }
    }

    pub fn run(&mut self, arg: DriveArg, butt_man: &mut ButtonMan) {
        arg.execute(self);
        butt_man.execute(*arg.get_button())
    }

    pub fn forwards(&mut self, precise: bool) {
        self.map(|x, _| x.move_voltage(Drive::cal_volt(if precise { Config::PRECISE_FORWARD_SPEED } else { Config::FORWARD_SPEED })).unwrap());
    }

    pub fn stop(&mut self) {
        self.map(|x, _| x.move_voltage(0).unwrap());
    }

    pub fn backwards(&mut self, precise: bool) {
        self.map(|x, _| x.move_voltage(Drive::cal_volt(-if precise { Config::PRECISE_BACKWARD_SPEED } else { Config::BACKWARD_SPEED })).unwrap())
    }

    pub fn left(&mut self, precise: bool) {
        let turnspeed: i8 = if precise { Config::PRECISE_TURN_SPEED } else { Config::TURN_SPEED };
        self.map(|x, i| {
            if i & 1 == 0 { // Right Motors
                x.move_voltage(Drive::cal_volt(turnspeed)).unwrap();
            } else { // Left Motors
                x.move_voltage(Drive::cal_volt(-turnspeed)).unwrap();
            }
        });
    }

    pub fn right(&mut self, precise: bool) {
        let turnspeed: i8 = if precise { Config::PRECISE_TURN_SPEED } else { Config::TURN_SPEED };
        self.map(|x, i| {
            if i & 1 == 0 { // Right Motors
                x.move_voltage(Drive::cal_volt(-turnspeed)).unwrap();
            } else { // Left Motors
                x.move_voltage(Drive::cal_volt(turnspeed)).unwrap();
            }
        });
    }

    fn map<F>(&mut self, f: F)
    where
        F: Fn(&mut Motor, u8),
    {
        f(&mut self.motor1, 1);
        f(&mut self.motor2, 2);
        f(&mut self.motor3, 3);
        f(&mut self.motor4, 4);
    }

    #[inline]
    pub fn cal_volt(speed: i8) -> i32 { 12000 * (speed as i32) / 100 } // Normalised speed from 1 to 100

    fn build_motor(id: u8) -> Motor {
        loop {
            if let Ok(x) = unsafe {
                Motor::new(
                    Config::MOTORS.id_to_port(id),
                    Config::GEAR_RATIO,
                    Config::MOTORS.units,
                    Config::MOTORS.id_to_reverse(id),
                )
            } { return x }
        }
    }
}