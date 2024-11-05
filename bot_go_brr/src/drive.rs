//! Drive code for the drive-train

use logic::warn;
use safe_vex::{controller::{self, Controller, ControllerAnalog}, imu, motor};
use crate::config;

/// Drives the drive-train based on user input and the angle integral and returns the thrust/y value (-12000.0..=12000)
pub fn user_control(angle_integral: &mut f32) -> i32 {
    // get the joystick values (from -127..=127)
    let j1x = controller::get_analog(Controller::Master, ControllerAnalog::LeftX).unwrap_or_default();
    let j1y = controller::get_analog(Controller::Master, ControllerAnalog::LeftY).unwrap_or_default();
    let j2x = controller::get_analog(Controller::Master, ControllerAnalog::RightX).unwrap_or_default();
    let j2y = controller::get_analog(Controller::Master, ControllerAnalog::RightY).unwrap_or_default();

    // get the current yaw of the robot
    let yaw = imu::get_yaw(config::IMU_PORT).unwrap_or_default();

    // calculate the left and right motor voltages
    let (ldr, rdr) = logic::drive::user_control(
        j1x as f32 / 127.0,
        j1y as f32 / 127.0,
        j2x as f32 / 127.0,
        j2y as f32 / 127.0,
        yaw as f32,
        config::TICK_SPEED as f32,
        angle_integral,
    );

    // drive the robot based on the ldr and rdr values
    voltage_left(ldr);
    voltage_right(rdr);

    // return the y value (for now)
    logic::magic::exp_daniel(j1y as f32 / 127.0) as i32
}

/// Sets the voltage of the left drive-train
pub fn voltage_left(voltage: i32) {
    // log any errors
    if let Err(err) = motor::move_voltage(config::motors::L1.port, config::motors::L1.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor L1's voltage: {err:?}");
    }
    if let Err(err) = motor::move_voltage(config::motors::L2.port, config::motors::L2.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor L2's voltage: {err:?}");
    }
    if let Err(err) = motor::move_voltage(config::motors::L3.port, config::motors::L3.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor L3's voltage: {err:?}");
    }
}

/// Sets the voltage of the right drive-train
pub fn voltage_right(voltage: i32) {
    // log any errors
    if let Err(err) = motor::move_voltage(config::motors::R1.port, config::motors::R1.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor R1's voltage: {err:?}");
    }
    if let Err(err) = motor::move_voltage(config::motors::R2.port, config::motors::R2.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor R2's voltage: {err:?}");
    }
    if let Err(err) = motor::move_voltage(config::motors::R3.port, config::motors::R3.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor R3's voltage: {err:?}");
    }
}
