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
        L1: 1, false;
        /// Bottom-left drive-train motor
        L2: 10, false;
        /// Top-right drive-train motor
        R1: 11, true;
        /// Bottom-right drive-train motor
        R2: 20, true;
    }

    /// the robot's turning speed (as a multiplier)
    pub const TURN_SPEED: f32 = 0.64;

    /// the multiplier for precise speed
    pub const PRECISE_MULTIPLIER: f32 = 0.60;
}

/// Daniel's magic number for the joysticks
#[allow(clippy::excessive_precision)]
pub const DMN: f32 = 1.02022606038826; // 12000 = 1024a^{127} - 1024

pub mod autonomous {
    use include_tt::include_tt;

    use crate::{ascii_bytecode, bytecode::ByteCode};

    /// The autonomous bytecode executed before a vex vrc match
    pub const MATCH_AUTO: &[ByteCode] = &include_tt!(ascii_bytecode! { #include_tt!("src/autonomous/match_auto.brb") });

    /// The autonomous bytecode executed during a vex vrc skills round
    pub const FULL_AUTO: &[ByteCode] = &include_tt!(ascii_bytecode! { #include_tt!("src/autonomous/full_auto.brb") });
}
