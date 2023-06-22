use vex_rt::motor::Motor;
use crate::config::Config;
use crate::button::ButtonArg;

#[derive(Debug)]
pub enum DriveArg {
    Forward(ButtonArg),
    Backward(ButtonArg),
    Left(ButtonArg),
    Right(ButtonArg),
    Stop(ButtonArg),
    Stall(ButtonArg),
}

impl DriveArg {
    pub fn execute(&self, drive: &mut Drive) {
        match self {
            DriveArg::Forward(_) => drive.forwards(),
            DriveArg::Backward(_) => drive.backwards(),
            DriveArg::Left(_) => drive.left(),
            DriveArg::Right(_) => drive.right(),
            DriveArg::Stop(_) => drive.stop(),
            DriveArg::Stall(_) => (),
        }
    }

    pub fn add(first: Self, second: Self) -> Self {
        match (first, second) {
            (x, DriveArg::Stop(_)) => x,
            (DriveArg::Stop(_), y) => y,
            (_, _) => DriveArg::Stall(ButtonArg::Null),
        }
    }

    pub fn to_string(&self) -> (&str, &str) {
        match self {
            DriveArg::Forward(x) => ("Forward", x.to_string()),
            DriveArg::Backward(x) => ("Backward", x.to_string()),
            DriveArg::Left(x) => ("Left", x.to_string()),
            DriveArg::Right(x) => ("Right", x.to_string()),
            DriveArg::Stop(x) => ("Stop", x.to_string()),
            DriveArg::Stall(x) => ("Stall", x.to_string()),
        }
    }

    pub fn get_misc(&self) -> &ButtonArg {
        match self {
            DriveArg::Forward(x) => x,
            DriveArg::Backward(x) => x,
            DriveArg::Left(x) => x,
            DriveArg::Right(x) => x,
            DriveArg::Stop(x) => x,
            DriveArg::Stall(x) => x,
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

    pub fn drive(&mut self, arg: DriveArg) { arg.execute(self); }

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
        }.unwrap_or_else(|_|
            panic!("Error: Could not configure / generate motor id '{0}' at port '{1}'!", id, Config::MOTORS.id_to_port(id))
        )
    }
}