//! Drive code for the drive-train

use drive_controls::pid::Pid;
use safe_vex::{imu, motor};
use crate::config;

/// Drives the drive-train based on x and y values and also an optional desired angle
pub fn drive(mut x: f32, y: f32, desired_angle: Option<f32>, rot_pid: &mut Pid<f32>) {
    // if the robot is not currently moving, then reset the inertial sensor
    if maths::absf(x) >= maths::absf(y) {
        let _ = imu::tare(config::IMU_PORT);
    }

    // if there is a desired angle then correct for it, *(and ignore the user-input x)*
    if let Some(angle) = desired_angle {
        // grab the inertial sensor yaw
        let yaw = imu::get_yaw(config::IMU_PORT).unwrap_or_else(|_| {
            safe_vex::io::println!("silent error: couldn't get yaw from imu"); // no silent fails
            0.0
        }) as f32;

        // correct for the sensor yaw by setting the new x value
        x = drive_controls::rot_correct(angle, yaw, rot_pid);
    }

    // apply the multipliers
    x *= config::TURN_MULTIPLIER;

    // pass them through arcade drive to get left and right drives
    let (ldr, rdr) = drive_controls::arcade(x as i32, y as i32);

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
