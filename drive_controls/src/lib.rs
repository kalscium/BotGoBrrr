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

pub use pid;

/// Creates a new PID
pub fn new_pid() -> pid::Pid<f32> {
    let mut pid = pid::Pid::new(0.0, 12000.0);

    // configure pid
    pid.p(1200.0, 12000.0);
    pid.i(120.0, 12000.0);
    pid.d(240.0, 12000.0);

    pid
}

/// Corrects the rotation of the robot based upon the yaw and the desired angle (returns the new x value)
pub fn rot_correct(angle: f32, yaw: f32, pid: &mut pid::Pid<f32>) -> f32 {
    // update internal PID values
    pid.setpoint(angle);

    // calculate correction x value
    let output = pid.next_control_output(yaw);

    // return it
    output.output
}

/// Finds the angle of the x and y values of the joystick
pub fn xy_to_angle(x: f32, y: f32) -> f32 {
    maths::atan(x / maths::absf(y + 0.0001))
}
