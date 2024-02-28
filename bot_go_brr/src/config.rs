use safe_vex::vex_rt::motor::{EncoderUnits, Gearset};
use crate::auto::Auto;

macro_rules! motor_config {
    ($(#[$meta:meta])* $motor:ident: $port:expr, $reverse:expr;) => {
        /// **(motor configuration)**
        ///
        $(#[$meta])*
        pub const $motor: MotorConfig = MotorConfig { port: $port, reverse: $reverse };
    };

    ($(#[$meta:meta])* $motor:ident: $port:expr, $reverse:expr; $($tail:tt)+) => {
        /// **(motor configuration)**
        ///
        $(#[$meta])*
        pub const $motor: MotorConfig = MotorConfig { port: $port, reverse: $reverse };
        motor_config! {
            $($tail)*
        }
    };
}

pub struct Config;
#[derive(Debug, Copy, Clone)]
pub struct MotorConfig {
    pub port: u8,
    pub reverse: bool,
}

/// DriveTrain Configs
impl Config {
    /// The gear ratio of the drive-train's motors
    pub const DRIVE_GEAR_RATIO: Gearset = Gearset::ThirtySixToOne;
    /// The unit used to mreasure the motor
    pub const DRIVE_UNIT: EncoderUnits = EncoderUnits::Degrees;

    motor_config! {
        /// Top-left drive-train motor
        DRIVE_L1: 5, false;
        /// Bottom-left drive-train motor
        DRIVE_L2: 3, false;
        /// Top-right drive-train motor
        DRIVE_R1: 20, true;
        /// Bottom-right drive-train motor
        DRIVE_R2: 16, true;
        
        /// Arm motor
        ARM: 15, true;
    }

    /// the robot's forward speed out of `100`
    pub const DRIVE_FORWARD_SPEED: u8 = 100;
    /// the robot's backward speed out of `100`
    pub const DRIVE_BACKWARD_SPEED: u8 = 90;
    /// the robot's turning speed out of `100`
    pub const DRIVE_TURN_SPEED: u8 = 60;
    /// the robot's arm speed out of `100`
    pub const ARM_SPEED: u8 = 75;

    /// The percentage of the normal drive speed for precise movement
    pub const PRECISE_SPEED: u8 = 60;
}

/// Misc Configs
impl Config {
    /// The minimum amount of activation the controller has to have to be activated
    pub const CONTROLLER_STICK_MIN: u8 = 0;
    /// The exponential multiplier for the joysticks
    pub const EXPO_MULTIPLIER: f64 = 1.0004f64;
}

/// Autonomous Algorithms
impl Config {
    // /// Competition Autonomous (before driver control)
    pub const AUTO_COMPETITION: Auto = Auto::new(
        &[(i32::MAX, 40), (0, 10), (-i32::MAX, 18), (0, 10), (i32::MAX, 20), (0, 10), (-i32::MAX, 40)],
        &[(i32::MAX, 40), (0, 10), (-i32::MAX, 18), (0, 10), (i32::MAX, 20), (0, 10), (-i32::MAX, 40)],
        &[(i32::MAX, 40), (0, 10), (-i32::MAX, 18), (0, 10), (i32::MAX, 20), (0, 10), (-i32::MAX, 40)],
        &[(i32::MAX, 40), (0, 10), (-i32::MAX, 18), (0, 10), (i32::MAX, 20), (0, 10), (-i32::MAX, 40)],
        &[], 
    );
}
