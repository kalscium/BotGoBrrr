//! Configurations for the robot

/// How long (in milliseconds) each 'tick' is
pub const TICK_SPEED: u32 = 10;

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

    /// The button to make the doinker spin 'upwards'
    pub const DOINKER_UP: ControllerDigital = ControllerDigital::A;
    /// The button to make the doinker spin 'downwards'
    pub const DOINKER_DOWN: ControllerDigital = ControllerDigital::B;

    /// The button to make the solenoid toggle
    pub const SOLENOID_TOGGLE: ControllerDigital = ControllerDigital::X;
}

/// Solenoid Configurations
pub mod solenoid {
    use safe_vex::port::AdiPort;

    /// The port of the solenoid
    pub const PORT: AdiPort = AdiPort::A;

    /// The delay (in ticks) between each solenoid actuation
    pub const DELAY: u32 = 8;
}

/// Configurations for autonomous
pub mod auton {
    use logic::pid::PIDConsts;
    use safe_vex::port::SmartPort;

    /// The port of the intertial sensor
    pub const IMU_PORT: safe_vex::port::SmartPort = safe_vex::port::SmartPort::Nine;

    /// The required minimum precision for the robot's angle during auton (must be within this angle)
    pub const ANGLE_PRECISION: f32 = 1.;

    /// The port of the left odom y rotation sensor
    pub const ODOM_LY_PORT: SmartPort = SmartPort::Eighteen;
    /// The port of the right odom y rotation sensor
    pub const ODOM_RY_PORT: SmartPort = SmartPort::Fourteen;

    /// The required minimum precision for the robot's coordinates (in mm) during auton (if you set this too low the robot will occilate forever)
    pub const ODOM_PRECISION: f32 = 4.;

    /// The max rotational error before saturation
    pub const MAX_ROT_ERR: f32 = 45.;

    /// The PID gain / configuration values for rotational/yaw corrections
    pub const ROT_PID: PIDConsts = PIDConsts {
        kp: 1. / MAX_ROT_ERR * 12000.,
        ki: 2., // decrease until oscillations are small enough
        prediction_window: 0.02, // decrease until the weird jittering stops
        saturation: 12000.,
    };

    /// The max y coordinate error in mm before saturation
    pub const MAX_Y_ERR: f32 = 300.;

    /// The PID gain / configuration values for y_coordinate corrections
    pub const Y_PID: PIDConsts = PIDConsts {
        kp: 1. / MAX_Y_ERR * 12000.,
        ki: 2., // decrease until oscillations are small enough
        prediction_window: 0.02, // decrease until the weird jittering stops
        saturation: 12000.,
    };
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
        port: safe_vex::port::SmartPort::Six,
        reverse: false,
    };

    /// The MotorConfig of the intake
    pub const INTAKE: MotorConfig = MotorConfig {
        port: safe_vex::port::SmartPort::Two,
        reverse: true,
    };

    /// The belt's speed in voltage
    pub const BELT_VOLTS: i32 = 12000 * 100 / 100;

    /// The MotorConfig of the doinker
    pub const DOINKER: MotorConfig = MotorConfig {
        port: safe_vex::port::SmartPort::One,
        reverse: false,
    };

    /// The doinker's speed in voltage
    pub const DOINKER_VOLTS: i32 = 12000 * 100 / 100;

    /// The front left motor
    pub const L1: MotorConfig = MotorConfig { port: SmartPort::Nineteen, reverse: true };
    /// The back left motor
    pub const L2: MotorConfig = MotorConfig { port: SmartPort::Thirteen, reverse: true };
    /// The top left motor (the extra 3rd one)
    pub const L3: MotorConfig = MotorConfig { port: SmartPort::Five, reverse: false };

    /// The front right motor
    pub const R1: MotorConfig = MotorConfig { port: SmartPort::Eleven, reverse: false };
    /// The back right motor
    pub const R2: MotorConfig = MotorConfig { port: SmartPort::Twelve, reverse: false };
    /// The top right motor (the extra 3rd one)
    pub const R3: MotorConfig = MotorConfig { port: SmartPort::Three, reverse: true };
}
