use vex_rt::prelude::*;
pub struct Config {}

impl Config {
    pub const TICK_SPEED: u64 = 50;
    /// Controller sensitivity
    pub const CONTROLLER_STICK_THRESHOLD: u8 = 12; // Controller sensitivity ?/127

    /// ### Motors
    pub const MOTORS: MotorConfig = MotorConfig {
        units: EncoderUnits::Rotations,

        motor1: 12,
        motor2: 3,
        motor3: 20,
        motor4: 5,

        reverse1: false,
        reverse2: true,
        reverse3: false,
        reverse4: true,
    };

    /// Port no work :C
    pub const _BROKEN_PORTS: [u8; 6] = [2, 6, 7, 8, 10, 11]; // Ports that don't work

    // Arm
    pub const ARM_PORT: u8 = 16;
    pub const ARM_SPEED: i8 = 100; // 100 / 100
    pub const ARM_RATIO: Gearset = Gearset::ThirtySixToOne;
    pub const ARM_REVERSE: bool = true;

    // Robot speeds
    pub const FORWARD_SPEED: i8 = 80; // 100 / 100
    pub const BACKWARD_SPEED: i8 = 80; // 80 / 100
    pub const TURN_SPEED: i8 = 40; // 50 / 100
    pub const GEAR_RATIO: Gearset = Gearset::ThirtySixToOne;

    // Robot precise speeds
    pub const PRECISE_FORWARD_SPEED: i8 = 40;
    pub const PRECISE_BACKWARD_SPEED: i8 = 40;
    pub const PRECISE_TURN_SPEED: i8 = 25;
}

/// ( Motor order goes from top to down, left to right )
pub struct MotorConfig {
    /// I don't know what this does
    pub units: EncoderUnits,
    
    // Ports
    pub motor1: u8,
    pub motor2: u8,
    pub motor3: u8,
    pub motor4: u8,

    // Reverseness
    pub reverse1: bool,
    pub reverse2: bool,
    pub reverse3: bool,
    pub reverse4: bool,
}

impl MotorConfig {
    pub fn id_to_port(&self, id: u8) -> u8 {
        match id {
            1 => self.motor1,
            2 => self.motor2,
            3 => self.motor3,
            4 => self.motor4,
            _ => MotorConfig::id_panic::<u8>(id),
        }
    }

    pub fn id_to_reverse(&self, id: u8) -> bool {
        match id {
            1 => self.reverse1,
            2 => self.reverse2,
            3 => self.reverse3,
            4 => self.reverse4,
            _ => MotorConfig::id_panic::<bool>(id),
        }
    }

    fn id_panic<T>(id: u8) -> T { panic!("Error: Motor id must be between 1 and 4! Given: {id}") }
}
