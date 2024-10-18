//! Drive code for the drive-train

use safe_vex::{imu, motor};
use crate::{config, maths};

/// Drives the drive-train based on x and y values
pub fn drive(x: f64, y: f64) {
    // grab the inertial sensor yaw
    let yaw = imu::get_yaw(config::IMU_PORT).unwrap_or_else(|_| {
        safe_vex::io::println!("silent error: couldn't get yaw from imu"); // no silent fails
        0.0
    });

    // calculate the course corrected x and y
    let (x, y) = course_correct(x, y, yaw);

    // pass them through arcade drive to get left and right drives
    let (ldr, rdr) = arcade(x as i32, y as i32);

    // drive
    voltage_left(ldr);
    voltage_right(rdr);
}

/// Sets the voltage of the left drive-train
pub fn voltage_left(voltage: i32) {
    // ignore all errors
    let _ = motor::move_voltage(config::motors::L1.port, config::motors::L1.reverse, voltage);
    let _ = motor::move_voltage(config::motors::L2.port, config::motors::L2.reverse, voltage);
    let _ = motor::move_voltage(config::motors::L3.port, config::motors::L3.reverse, voltage);
}

/// Sets the voltage of the right drive-train
pub fn voltage_right(voltage: i32) {
    // ignore all errors
    let _ = motor::move_voltage(config::motors::R1.port, config::motors::R1.reverse, voltage);
    let _ = motor::move_voltage(config::motors::R2.port, config::motors::R2.reverse, voltage);
    let _ = motor::move_voltage(config::motors::R3.port, config::motors::R3.reverse, voltage);
}

/// Performs an arcade drive transformation on x and y values to produce ldr and rdr valueso
pub fn arcade(x: i32, y: i32) -> (i32, i32) {
    let ldr = (y + x).clamp(-12000, 12000);
    let rdr = (y - x).clamp(-12000, 12000);

    (ldr, rdr)
}

/// Course corrects the x and y values based on the interial sensor yaw
pub fn course_correct(x: f64, y: f64, yaw: f64) -> (f64, f64) {
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
