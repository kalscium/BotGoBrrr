//! Configurations for the robot

/// How long (in milliseconds) each 'tick' is
pub const TICK_SPEED: u32 = 50;

/// The multiplier applied to the robot's turning
pub const TURN_MULTIPLIER: f32 = 0.64;

/// The multiplier applied to the robot while it's in it's 'precise' mode
pub const PRECISE_MULTIPLIER: f32 = 0.64;

/// Control Configurations
pub mod controls {
    use safe_vex::controller::ControllerDigital;

    /// The button to make the belt spin 'upwards'
    pub const BELT_UP: ControllerDigital = ControllerDigital::R2;
    /// The button to make the belt spin 'downwards'
    pub const BELT_DOWN: ControllerDigital = ControllerDigital::R1;

    /// The button to make the solenoid toggle
    pub const SOLENOID_TOGGLE: ControllerDigital = ControllerDigital::X;
}

/// The port of the intertial sensor
pub const IMU_PORT: safe_vex::port::SmartPort = safe_vex::port::SmartPort::Twenty;

/// Solenoid Configurations
pub mod solenoid {
    use safe_vex::port::AdiPort;

    /// The port of the solenoid
    pub const PORT: AdiPort = AdiPort::A;

    /// The delay (in ticks) between each solenoid actuation
    pub const DELAY: u32 = 8;
}

/// Odometry Configurations
pub mod odom {
    use safe_vex::port::SmartPort;

    /// The port of the x rotation sensor
    pub const PORT_X: SmartPort = SmartPort::Ten;
}

/// Configurations for autonomous
pub mod auton {
    /// The required minimum precision for the robot's angle during auton (must be within this angle)
    pub const ANGLE_PRECISION: f32 = 1.0;
    /// The path of the file used for recording
    pub const RECORD_PATH: &str = "/usd/record.rs";
}

/// Configurations for logging
pub mod log {
    use logic::log::Level;

    /// The logfile path for opcontrol
    pub const LOGFILE_OP_PATH: &str = "/usd/bgb_logs_opctrl.txt";
    /// The logfile path for autonomous
    pub const LOGFILE_AUTO_PATH: &str = "/usd/bgb_logs_auton.txt";
    /// The minimum log level for stdout logs
    pub const STDOUT_MIN: Level = Level::Info;
    /// THe minimum log level for logfile logs
    pub const LOGFILE_MIN: Level = Level::Debug;
}

/// Motor Configurations
pub mod motors {
    use safe_vex::port::SmartPort;

    /// A motor's configurations
    pub struct MotorConfig {
        pub port: SmartPort,
        pub reverse: bool,
    }

    /// The MotorConfig of the belt
    pub const BELT: MotorConfig = MotorConfig {
        port: safe_vex::port::SmartPort::Twelve,
        reverse: false,
    };

    /// The belt's speed in voltage
    pub const BELT_SPEED: i32 = 12000 * 62 / 100;

    /// The front left motor
    pub const L1: MotorConfig = MotorConfig { port: SmartPort::Sixteen, reverse: false };
    /// The back left motor
    pub const L2: MotorConfig = MotorConfig { port: SmartPort::Nineteen, reverse: false };
    /// The top left motor (the extra 3rd one)
    pub const L3: MotorConfig = MotorConfig { port: SmartPort::Eighteen, reverse: false };

    /// The front right motor
    pub const R1: MotorConfig = MotorConfig { port: SmartPort::Nine, reverse: true };
    /// The back right motor
    pub const R2: MotorConfig = MotorConfig { port: SmartPort::Four, reverse: true };
    /// The top right motor (the extra 3rd one)
    pub const R3: MotorConfig = MotorConfig { port: SmartPort::Five, reverse: true };
}
