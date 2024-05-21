macro_rules! motor_config {
    ($(#[$meta:meta])* $motor:ident: $port:expr, $reverse:expr;) => {
        /// **(motor configuration)**
        ///
        $(#[$meta])*
        pub const $motor: $crate::config::MotorConfig = $crate::config::MotorConfig { port: $port, reverse: $reverse };
    };

    ($(#[$meta:meta])* $motor:ident: $port:expr, $reverse:expr; $($tail:tt)+) => {
        /// **(motor configuration)**
        ///
        $(#[$meta])*
        pub const $motor: $crate::config::MotorConfig = $crate::config::MotorConfig { port: $port, reverse: $reverse };
        motor_config! {
            $($tail)*
        }
    };
}

#[derive(Debug, Copy, Clone)]
pub struct MotorConfig {
    pub port: u8,
    pub reverse: bool,
}

pub mod drive {
    use safe_vex::vex_rt::motor::{EncoderUnits, Gearset};

    /// The gear ratio of the drive-train's motors
    pub const GEAR_RATIO: Gearset = Gearset::ThirtySixToOne;
    /// The unit used to mreasure the motor
    pub const UNIT: EncoderUnits = EncoderUnits::Degrees;

    motor_config! {
        /// Top-left drive-train motor
        L1: 12, false;
        /// Bottom-left drive-train motor
        L2: 20, false;
        /// Top-right drive-train motor
        R1: 1, true;
        /// Bottom-right drive-train motor
        R2: 9, true;

        /// Belt motor
        BELT: 21, false;
    }

    // /// the robot's forward speed out of `100`
    // pub const FORWARD_SPEED: u8 = 100;
    // /// the robot's backward speed out of `100`
    // pub const BACKWARD_SPEED: u8 = 90;
    // /// the robot's turning speed out of `100`
    // pub const TURN_SPEED: u8 = 60;
    /// The robot's conveyor belt voltage out of `12000`
    pub const BELT_VOLTAGE: i32 = 12000;

    // /// The percentage of the normal drive speed for precise movement
    // pub const PRECISE_SPEED: u8 = 60;
}

/// The minimum amount of activation the controller has to have to be activated
pub const CONTROLLER_STICK_MIN: u8 = 10;

/// The exponential multiplier for the joysticks
pub const EXPONENT_SPEED: f32 = 16.0;

pub mod autonomous {
    use crate::{ascii_bytecode, bytecode::ByteCode};

    /// The autonomous bytecode executed before a competition match
    pub const COMP_AUTO: [ByteCode; 0] = ascii_bytecode! {};

    // /// The autonomous bytecode executed for skills
    pub const FULL_AUTO: [ByteCode; 0] = ascii_bytecode! {};
}
