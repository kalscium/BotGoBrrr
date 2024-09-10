use safe_vex::adi::AdiPort;

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
        L1: 15, false;
        /// Bottom-left drive-train motor
        L2: 18, false;
        /// Top-right drive-train motor
        R1: 9, true;
        /// Bottom-right drive-train motor
        R2: 4, true;

        /// Belt motor
        BELT: 12, false;
        /// Intake motor
        INTAKE: 10, false;
    }

    /// the robot's turning speed (as a multiplier)
    pub const TURN_SPEED: f32 = 0.64;

    /// The robot's conveyor belt voltage out of `12000`
    pub const BELT_VOLTAGE: i32 = 5400;
    /// The robot's graber motor's voltage out of `12000` when moving down
    pub const GRABER_VOLTAGE_DOWN: i32 = 12000;
    /// The robot's graber motor's voltage out of `12000` when moving up
    pub const GRABER_VOLTAGE_UP: i32 = 2880;
    /// the multiplier for precise speed
    pub const PRECISE_MULTIPLIER: f32 = 0.60;
}

/// The adi port of the pneumatics solanoid
pub const SOLENOID_PORT: AdiPort = AdiPort::A;
/// A tick delay of the solanoid
pub const SOLENOID_DELAY: u16 = 8;

/// Daniel's magic number for the joysticks
pub const DMN: f32 = 1.0195691192404441; // 12000 = 1024a^{x} - 1024

/// The threshold for being 'zero' for the controller joystick (to combat stick drift) (used for robot controls)
pub const STICK_RESET_THRESHOLD: u8 = 32;

pub mod autonomous {
    use include_tt::include_tt;

    use crate::{ascii_bytecode, bytecode::ByteCode};

    /// The autonomous bytecode executed before a vex vrc match
    pub const MATCH_AUTO: &[ByteCode] = &include_tt!(ascii_bytecode! { #include_tt!("src/autonomous/match_auto.brb") });

    /// The autonomous bytecode executed during a vex vrc skills round
    pub const FULL_AUTO: &[ByteCode] = &include_tt!(ascii_bytecode! { #include_tt!("src/autonomous/full_auto.brb") });
}
