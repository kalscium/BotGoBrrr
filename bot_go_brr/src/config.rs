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
        L2: 14, false;
        /// Top-right drive-train motor
        R1: 20, true;
        /// Bottom-right drive-train motor
        R2: 4, true;

        /// Belt motor
        BELT: 18, false;
        /// Inserter motor
        INSERTER: 10, false;
        /// Graber motor
        GRABER: 16, false;
    }

    /// the robot's turning speed (as a multiplier)
    pub const TURN_SPEED: f32 = 0.64;

    /// The robot's conveyor belt voltage out of `12000`
    pub const BELT_VOLTAGE: i32 = 7200;
    /// The robot's graber motor's voltage out of `12000` when moving down
    pub const GRABER_VOLTAGE_DOWN: i32 = 12000;
    /// The robot's graber motor's voltage out of `12000` when moving up
    pub const GRABER_VOLTAGE_UP: i32 = 2880;
    /// the multiplier for precise speed
    pub const PRECISE_MULTIPLIER: f32 = 0.40;
}

/// The minimum amount of activation the controller has to have to be activated
pub const CONTROLLER_STICK_MIN: u8 = 10;

/// Daniel's magic number for the joysticks
pub const DMN: f32 = 0.00000000000000000000000000000262; // 12000 = x * 127^{16}

pub mod autonomous {
    use include_tt::include_tt;

    use crate::{ascii_bytecode, bytecode::ByteCode};

    /// The autonomous bytecode executed before a vex vrc match
    pub const MATCH_AUTO: &[ByteCode] = &include_tt!(ascii_bytecode! { #include_tt!("src/autonomous/match_auto.brb") });

    /// The autonomous bytecode executed during a vex vrc skills round
    pub const FULL_AUTO: &[ByteCode] = &include_tt!(ascii_bytecode! { #include_tt!("src/autonomous/full_auto.brb") });
}
