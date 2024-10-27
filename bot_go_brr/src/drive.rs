//! Drive code for the drive-train

use safe_vex::{controller::{self, Controller, ControllerAnalog}, motor};
use crate::config;

/// Drives the drive-train based on user input and the angle integral and returns the thrust/y value (-12000.0..=12000)
pub fn user_control(angle_integral: &mut f32) -> i32 {
    // get the joystick values (from -127..=127)
    let j1x = controller::get_analog(Controller::Master, ControllerAnalog::LeftX).unwrap_or_default();
    let j1y = controller::get_analog(Controller::Master, ControllerAnalog::LeftY).unwrap_or_default();
    let j2x = controller::get_analog(Controller::Master, ControllerAnalog::RightX).unwrap_or_default();
    let j2y = controller::get_analog(Controller::Master, ControllerAnalog::RightY).unwrap_or_default();

    // get the calculated voltages
    let j1xv = drive_controls::exp_daniel(j1x as f32 / 127.0);
    let j1yv = drive_controls::exp_daniel(j1y as f32 / 127.0);


    // if the second joystick is active, then derive an angle from it's x and y values and drive based of that instead
    if j2x != 0 || j2y != 0 {
        // get the target angle (from x and y)
        let target_angle = drive_controls::xy_to_angle(j2x as f32, j2y as f32);

        // drive
        drive_exact(j1yv, target_angle, angle_integral);
    } else {
        // drive normally
        drive(j1xv, j1yv);
    }

    // return the y voltage (for now)
    j1yv as i32
}

/// Drives the drive-train based on x and y values
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
