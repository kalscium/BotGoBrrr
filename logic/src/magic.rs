//! A bunch of magic numbers and algorithms

/// Daniels magic number for nice, smooth and exponential controls (https://www.desmos.com/calculator/rgkyn8zesy) or (`y = 1024a^{x} - 1024` when `(127, 12000)`)
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

/// Ethan's magic number for precise, smooth and exponential corrections (https://www.desmos.com/calculator/upgbzz4cz5) or (`y = 128a^{x} - 128` when `(1, 12000)`)
pub const EMN: f32 = 94.75;

/// Passes x (`-1..=1`) through ethan's algorithm to produce an exponential voltage from `-12000..=12000`
pub fn exp_ethan(x: f32) -> f32 {
    (128.0 * maths::powf(EMN, maths::absf(x)) - 128.0) // main part of the equation
        * maths::signumf(x) // to maintain the sign
}


/// Passes x (`-1..=1`) through ethan's algorithm to produce a log voltage from `-12000..=12000`
pub fn log_ethan(x: f32) -> f32 {
    (-128.0 * maths::powf(EMN, 1.0-maths::absf(x)) + 128.0 * EMN) // main part of the equation
        * maths::signumf(x) // to maintain the sign
}
