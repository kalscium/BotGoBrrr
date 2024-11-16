//! Drive code for the drive-train

use logic::warn;
use safe_vex::{controller::{self, Controller, ControllerAnalog}, imu, motor, port::SmartPort, rotation};
use crate::config;

/// Gets the current yaw of the robot
pub fn get_yaw() -> f32 {
    match imu::get_yaw(config::auton::IMU_PORT) {
        Ok(yaw) => yaw as f32,
        Err(err) => {
            warn!("`PROSErr` encountered while getting imu yaw: {err:?}");
            0.0
        },
    }
}

/// Gets the angle reading from a rotation sensor at the specified port
pub fn get_rotation_angle(port: SmartPort) -> f32 {
    match rotation::get_angle(port) {
        Ok(angle) => angle as f32 / 100.,
        Err(err) => {
            warn!("`PROSErr` encountered while getting angle of rotation sensor at port `{port:?}`: `{err:?}`");
            0.0
        },
    }
}

/// Drives the drive-train based on user input and previous voltage drive
pub fn user_control(prev_vdr: &mut (i32, i32)) -> i32 {
    // get the joystick values (from -127..=127)
    let j1x = controller::get_analog(Controller::Master, ControllerAnalog::LeftX).unwrap_or_default();
    let j1y = controller::get_analog(Controller::Master, ControllerAnalog::LeftY).unwrap_or_default();
    let j2x = controller::get_analog(Controller::Master, ControllerAnalog::RightX).unwrap_or_default();
    let j2y = controller::get_analog(Controller::Master, ControllerAnalog::RightY).unwrap_or_default();

    // get the current yaw of the robot
    let yaw = get_yaw();

    // calculate the left and right motor voltages
    let (ldr, rdr) = logic::drive::user_control(
        config::TURN_MULTIPLIER,
        j1x as f32 / 127.0,
        j1y as f32 / 127.0,
        j2x as f32 / 127.0,
        j2y as f32 / 127.0,
        yaw,
        prev_vdr,
    );

    // drive the robot based on the ldr and rdr values
    voltage_left(ldr);
    voltage_right(rdr);

    // return the thrust (for now)
    logic::magic::exp_daniel(j1y as f32 / 127.) as i32
}

/// Sets the voltage of the left drive-train
pub fn voltage_left(voltage: i32) {
    // log any errors
    if let Err(err) = motor::move_voltage(config::motors::L1.port, config::motors::L1.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor L1's voltage at port {}: {err:?}", config::motors::L1.port as u8);
    }
    if let Err(err) = motor::move_voltage(config::motors::L2.port, config::motors::L2.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor L2's voltage at port {}: {err:?}", config::motors::L2.port as u8);
    }
    if let Err(err) = motor::move_voltage(config::motors::L3.port, config::motors::L3.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor L3's voltage at port {}: {err:?}", config::motors::L3.port as u8);
    }
}

/// Sets the voltage of the right drive-train
pub fn voltage_right(voltage: i32) {
    // log any errors
    if let Err(err) = motor::move_voltage(config::motors::R1.port, config::motors::R1.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor R1's voltage at port {}: {err:?}", config::motors::R1.port as u8);
    }
    if let Err(err) = motor::move_voltage(config::motors::R2.port, config::motors::R2.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor R2's voltage at port {}: {err:?}", config::motors::R2.port as u8);
    }
    if let Err(err) = motor::move_voltage(config::motors::R3.port, config::motors::R3.reverse, voltage) {
        warn!("`PROSErr` occured while setting drivetrain motor R3's voltage at port {}: {err:?}", config::motors::R3.port as u8);
    }
}
