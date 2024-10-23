//! A library for generating the motor voltages of a drive-train from controller inputs

#![no_std]

/// Performs an arcade drive transformation on x and y values (`-12000..=12000`) to produce left and right drive voltages
pub fn arcade(x: i32, y: i32) -> (i32, i32) {
    let ldr = (y + x).clamp(-12000, 12000);
    let rdr = (y - x).clamp(-12000, 12000);

    (ldr, rdr)
}

/// Daniels magic number for nice, smooth and exponential controls (`12000 = 1024a^{1}`)
const DMN: f32 = 12.71875;

/// Passes x (`-1..=1`) through daniel's algorithm to produce an exponential voltage from `-12000..=12000`
pub fn exp_daniel(x: f32) -> f32 {
    (1024.0 * maths::powf(DMN, maths::absf(x)) - 1024.0) // main part of the equation
        * maths::signumf(x) // to maintain the sign
}

/// Passes x (`-1..=1`) through daniel's algorithm to produce a log voltage from `-12000..=12000`
pub fn log_daniel(x: f32) -> f32 {
    (-1024.0 * maths::powf(DMN, maths::absf(x)) + 1024.0 * DMN)
        * maths::signumf(x) // to maintain the sign
}

/// Finds the lowest difference in angle between two angles (-180..=180 (for both arguments and return value))
pub fn low_angle_diff(x: f32, y: f32) -> f32 {
    // get the first possible difference in angle
    let diff1 = x - y;
    // get the second possible difference in angle
    let diff2 = (360.0 - maths::absf(x) - maths::absf(y))
        * maths::signumf(y);

    // return the smaller difference
    if maths::absf(diff2) < maths::absf(diff1) {
        diff2
    } else {
        diff1 // slight preference for diff 1 but still the same distance travelled
    }
}

/// Corrects the rotation of the robot based upon the error (difference in) angle (-180..=180) and returns the new x value
pub fn rot_correct(diff: f32) -> f32 {
    // calculate correction x value with daniel's magic number
    let xc = exp_daniel(diff / 180.0);

    // return it
    xc
}

/// Finds the angle of the x and y values of the joystick according to the top of the joystick
pub fn xy_to_angle(x: f32, y: f32) -> f32 {
    if y < 0.0 {
        maths::atan(maths::absf(y) / (x + 0.0001 * maths::signumf(x)))
            + 90.0 * maths::signumf(x)
    } else {
        maths::atan(x / (y + 0.0001 * maths::signumf(y)))
    }.clamp(-179.0, 179.0)
}
