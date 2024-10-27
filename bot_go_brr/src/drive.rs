//! Drive code for the drive-train

use safe_vex::motor;
use crate::config;

/// Drives the drive-train based on x and y values and also an optional desired angle
pub fn drive(mut x: f32, y: f32) {
    // apply the multipliers
    x *= config::TURN_MULTIPLIER;

    // pass them through arcade drive to get left and right drives
    let (ldr, rdr) = drive_controls::arcade(x as i32, y as i32);

    // drive
    voltage_left(ldr);
    voltage_right(rdr);
}

/// Drive the robot based on exact yaw values (and eventually location values aswell)
pub fn drive_exact(
    y: f32, // will be replaced later with target location x and y values
    target_angle: f32,
    angle_integral: &mut f32
) {
    // grab the inertial sensor yaw
    let yaw = unsafe {
        safe_vex::bindings::imu_get_yaw(config::IMU_PORT as u8)
    } as f32;

    // find the error in the angle and target angle
    let error = drive_controls::low_angle_diff(target_angle, yaw);

    // correct for the error by setting the new x value
    let x = drive_controls::rot_correct(error, config::TICK_SPEED as f32, angle_integral);
    safe_vex::io::println!("angle: {target_angle}\nyaw: {yaw}\nx: {x}");

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
