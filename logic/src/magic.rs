//! A bunch of magic numbers and algorithms

/// Daniels magic number for nice, smooth and exponential controls (`y = 1024a^{x} - 1024`)
pub const DMN: f32 = 12.71875;

/// Passes x (`-1..=1`) through daniel's algorithm to produce an exponential voltage from `-12000..=12000`
pub fn exp_daniel(x: f32) -> f32 {
    (1024.0 * maths::powf(DMN, maths::absf(x)) - 1024.0) // main part of the equation
        * maths::signumf(x) // to maintain the sign
}

/// Passes x (`-1..=1`) through daniel's algorithm to produce a log voltage from `-12000..=12000`
pub fn log_daniel(x: f32) -> f32 {
    (-1024.0 * maths::powf(DMN, 1.0-maths::absf(x)) + 1024.0 * DMN)
        * maths::signumf(x) // to maintain the sign
}

/// Ethan's magic number for precise, smooth and exponential corrections (`y = a^{x}/56 - 1/56`)
pub const EMN: f32 = 57.0;

/// Passes x (`-1..=1`) through ethan's algorithm to produce an exponential number from `-1..=1`
pub fn exp_ethan(x: f32) -> f32 {
    (maths::powf(EMN, maths::absf(x)) / 56.0 - 1.0/56.0) // main part of the equation
        * maths::signumf(x) // to maintain the sign
}

/// Passes x (`-1..=1`) through ethan's algorithm to produce an logarithmic number from `-1..=1`
pub fn log_ethan(x: f32) -> f32 {
    (maths::powf(EMN, 1.0-maths::absf(x)) / -56.0 + 1.0/56.0 * EMN) // main part of the equation
        * maths::signumf(x) // to maintain the sign
}
