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

/// Course corrects the x and y values (`-12000..=12000`) based on the interial sensor yaw
pub fn course_correct(x: f32, y: f32, yaw: f32) -> (f32, f32) {
    // find the angle that the x and y make through the origin
    let angle = maths::atan(x / (y + 1.0));

    // find the difference in angles between the angle and the yaw
    let diff = angle - yaw * maths::signumf(y);

    // calculate the course correct based on the difference
    let new_x = diff / 45.0
        * (maths::absf(x) + maths::absf(y)) / 2.0 // find the average absolute value of x and y
        * (maths::signumf(y)); // ???

    (new_x, y)
}
