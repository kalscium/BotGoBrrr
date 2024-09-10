pub const STICK_RESET_THRESHOLD: u8 = 32;

/// Daniel's magic number for the joysticks
#[allow(clippy::excessive_precision)]
pub const DMN: f32 = 1.02022606038826; // 12000 = 1024a^{127} - 1024

pub mod drive {
    pub const TURN_SPEED: f32 = 0.64;
    pub const PRECISE_MULTIPLIER: f32 = 0.60;
}
