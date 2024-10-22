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

/// Course corrects the x and y values (`-12000..=12000`) based on the interial sensor yaw and PID
pub fn course_correct(x: f32, y: f32, yaw: f32, pid: &mut pid::Pid<f32>) -> (f32, f32) {
    // find the angle that the x and y make through the origin
    let angle = maths::atan(x / (y + 1.0));

    // update internal PID values
    pid.setpoint(angle);

    // calculate the course correct based on the PID
    let output = pid.next_control_output(yaw);
    let new_x = output.output;

    (new_x, y)
}
