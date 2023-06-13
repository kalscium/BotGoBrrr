use vex_rt::motor::Motor;
use crate::config::Config;

#[derive(Debug)]
pub enum Arg {
    Forward,
    Backward,
    Left,
    Right,
    Stop,
    Stall,
}

impl Arg {
    pub fn execute(&self, drive: &mut Drive) {
        match self {
            Arg::Forward => drive.forwards(),
            Arg::Backward => drive.backwards(),
            Arg::Left => drive.left(),
            Arg::Right => drive.right(),
            Arg::Stop => drive.stop(),
            Arg::Stall => (),
        }
    }

    pub fn add(first: Self, second: Self) -> Self {
        match (first, second) {
            (x, Arg::Stop) => x,
            (Arg::Stop, y) => y,
            (_, _) => Arg::Stall,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Arg::Forward => "Forward",
            Arg::Backward => "Backward",
            Arg::Left => "Left",
            Arg::Right => "Right",
            Arg::Stop => "Stop",
            Arg::Stall => "Stall",
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

    pub fn drive(&mut self, arg: Arg) { arg.execute(self); }

    pub fn forwards(&mut self) {
        self.map(|x, _| x.move_i8(Drive::cal_volt(Config::FORWARD_SPEED)).unwrap());
    }

    pub fn stop(&mut self) {
        self.map(|x, _| x.move_i8(0).unwrap());
    }

    pub fn backwards(&mut self) {
        self.map(|x, _| x.move_i8(Drive::cal_volt(-Config::BACKWARD_SPEED)).unwrap())
    }

    pub fn left(&mut self) {
        self.map(|x, i| {
            if i & 1 == 0 { // Right Motors
                x.move_i8(Drive::cal_volt(Config::TURN_SPEED)).unwrap();
            } else { // Left Motors
                x.move_i8(Drive::cal_volt(-Config::TURN_SPEED)).unwrap();
            }
        });
    }

    pub fn right(&mut self) {
        self.map(|x, i| {
            if i & 1 == 0 { // Right Motors
                x.move_i8(Drive::cal_volt(-Config::TURN_SPEED)).unwrap();
            } else { // Left Motors
                x.move_i8(Drive::cal_volt(Config::TURN_SPEED)).unwrap();
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

    fn cal_volt(speed: i8) -> i8 { (127i16 * speed as i16 / 100i16) as i8 }

    fn build_motor(id: u8) -> Motor {
        unsafe {
            Motor::new(
                Config::MOTORS.id_to_port(id),
                Config::GEAR_RATIO,
                Config::MOTORS.units,
                Config::MOTORS.id_to_reverse(id),
            )
        }.expect("Error: Could not configure / generate motor!")
    }
}