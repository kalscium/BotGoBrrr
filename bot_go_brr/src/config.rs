use safe_vex::vex_rt::motor::{EncoderUnits, Gearset};

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
        DRIVE_L1: 1, false;
        /// Bottom-left drive-train motor
        DRIVE_L2: 3, false;
        /// Top-right drive-train motor
        DRIVE_R1: 2, false;
        /// Bottom-right drive-train motor
        DRIVE_R2: 4, false;
    }

    /// the robot's forward speed out of `100`
    pub const DRIVE_FORWARD_SPEED: u8 = 100;
    /// the robot's backward speed out of `100`
    pub const DRIVE_BACKWARD_SPEED: u8 = 90;
    /// the robot's turning speed out of `100`
    pub const DRIVE_TURN_SPEED: u8 = 50;
    /// the robot's strafe speed out of `100`
    pub const DRIVE_STRAFE_SPEED: u8 = 60;
}

/// Misc Configs
impl Config {
    /// The minimum amount of activation the controller has to have to be activated
    pub const CONTROLLER_STICK_MIN: u8 = 8;
}