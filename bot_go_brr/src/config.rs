//! Configurations for the robot

// rewrite this at somepoint if it gets too messy

// pull the static toml configurations
static_toml::static_toml! {
    const CONFIGS = include_toml!("robot-configs.toml");
}

/// How long (in milliseconds) each 'tick' is
pub const TICK_SPEED: u32 = CONFIGS.general.tick_speed as u32;

/// The multiplier applied to the robot's turning
pub const TURN_MULTIPLIER: f64 = CONFIGS.drive.turn_multiplier;

/// The multiplier applied to the robot while it's in it's 'precise' mode
pub const PRECISE_MULTIPLIER: f64 = CONFIGS.drive.precise_multiplier;

/// Daniel's Magic Number for nice, smooth exponential controls
pub const DMN: f64 = CONFIGS.drive.dmn;

/// Solenoid Configurations
pub mod solenoid {
    use safe_vex::port::AdiPort;
    use super::CONFIGS;

    const fn u8_to_adiport(port: u8) -> safe_vex::port::AdiPort {
        unsafe {
            let pointer = &port as *const u8 as *const safe_vex::port::AdiPort;
            *pointer
        }
    }

    /// The port of the solenoid
    pub const PORT: AdiPort = u8_to_adiport(CONFIGS.solenoid.port as u8);
    /// The delay (in ticks) between each solenoid actuation
    pub const DELAY: u32 = CONFIGS.solenoid.delay as u32;
}

/// Motor Configurations
pub mod motors {
    use safe_vex::port::SmartPort;
    use super::CONFIGS;

    /// A motor's configurations
    pub struct MotorConfig {
        pub port: SmartPort,
        pub reverse: bool,
    }

    const fn u8_to_smartport(port: u8) -> safe_vex::port::SmartPort {
        unsafe {
            let pointer = &port as *const u8 as *const safe_vex::port::SmartPort;
            *pointer
        }
    }

    /// The belt motor
    pub const BELT: MotorConfig = MotorConfig { port: u8_to_smartport(CONFIGS.belt.port as u8), reverse: CONFIGS.belt.reverse };
    /// The belt's speed in voltage
    pub const BELT_SPEED: i32 = CONFIGS.belt.speed as i32;

    /// The front left motor
    pub const L1: MotorConfig = MotorConfig { port: u8_to_smartport(CONFIGS.drive.l_1.port as u8), reverse: CONFIGS.drive.l_1.reverse };
    /// The back left motor
    pub const L2: MotorConfig = MotorConfig { port: u8_to_smartport(CONFIGS.drive.l_2.port as u8), reverse: CONFIGS.drive.l_2.reverse };
    /// The top left motor (the extra 3rd one)
    pub const L3: MotorConfig = MotorConfig { port: u8_to_smartport(CONFIGS.drive.l_3.port as u8), reverse: CONFIGS.drive.l_3.reverse };

    /// The front right motor
    pub const R1: MotorConfig = MotorConfig { port: u8_to_smartport(CONFIGS.drive.r_1.port as u8), reverse: CONFIGS.drive.r_1.reverse };
    /// The back right motor
    pub const R2: MotorConfig = MotorConfig { port: u8_to_smartport(CONFIGS.drive.r_2.port as u8), reverse: CONFIGS.drive.r_2.reverse };
    /// The top right motor (the extra 3rd one)
    pub const R3: MotorConfig = MotorConfig { port: u8_to_smartport(CONFIGS.drive.r_3.port as u8), reverse: CONFIGS.drive.r_3.reverse };
}
