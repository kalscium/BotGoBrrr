/// A macro for printing debug information
macro_rules! debug {
    ($debug_info:ident: $($tt:tt)*) => {
        $debug_info.push(format!($($tt)*));
    };
}

/// Passes through a joystick's x and y values through all of the drive controls and returns the left and right drives and also any debug information
pub fn controls(x: f32, y: f32, yaw: f32) -> (i32, i32, Vec<String>) {
    let mut debug_info = Vec::new();

    // get the voltage values
    let xv = drive_controls::exp_daniel(x);
    let yv = drive_controls::exp_daniel(y);

    // course correct
    let (xvc, yvc) = drive_controls::course_correct(xv, yv, yaw);

    // get the final left and right drive voltages
    let (ldr, rdr) = drive_controls::arcade(xvc as i32, yvc as i32);

    // print the debug information
    debug!(debug_info: "joystick x: {x}");
    debug!(debug_info: "joystick y: {y}");
    debug!(debug_info: "joyvolt  x: {xv}");
    debug!(debug_info: "joyvolt  y: {yv}\n");
    debug!(debug_info: "yaw       : {yaw}");
    debug!(debug_info: "correct  x: {xvc}");
    debug!(debug_info: "correct  y: {yvc}\n");

    debug!(debug_info: "(ldr, rdr): ({ldr}, {rdr})");

    // return them
    (ldr, rdr, debug_info)
}
