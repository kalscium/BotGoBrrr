//! Functions for driving the robot (calculations & drivetrain)

const std = @import("std");
const Motor = @import("Motor.zig");
const port = @import("port.zig");
const pros = @import("pros");
const def = @import("def.zig");
const options = @import("options");
const controller = @import("controller.zig");

/// Driving in reverse toggle button
pub const reverse_toggle: c_int = pros.misc.E_CONTROLLER_DIGITAL_LEFT;

/// Drivetrain motor configs
pub const drivetrain_motors = struct {
    // The order for drivetrain motors is as follows,
    // l0 are the motors of the left side of the drivetrain.
    // r0 are the motors of the right side of the drivetrain.
    // x1 are the front motors of the drivetrain
    // x2 are the back motors of the drivetrain
    // x3 are the top motors of the drivetrain
    // 
    // Also, the tower is at the **FRONT** of the robot
    pub const l1 = drivetrainMotor(-16);
    pub const l2 = drivetrainMotor(-10);
    pub const l3 = drivetrainMotor(9);
    pub const r1 = drivetrainMotor(11);
    pub const r2 = drivetrainMotor(2);
    pub const r3 = drivetrainMotor(-1);
};

/// The multiplier applied to the robot's turning & movement speed normally
pub const drive_multiplier = 1.0;
pub const turn_multiplier = 1.0;

/// Reads the controller and updates the drivetrain accordingly based upon the
/// enabled build options
/// 
/// Also takes in whether the robot is currently driving backwards (in reverse)
/// 
/// Updates the port buffer on any motor disconnects
pub fn controllerUpdate(reverse: *bool, port_buffer: *port.PortBuffer) void {
    // hopefully gets set by one of the options
    var ldr: i32 = 0;
    var rdr: i32 = 0;

    if (comptime options.toggle_arcade) {
        ldr, rdr = toggleArcade();
    } else {
        // get the normalized main joystick values
        const j1 = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127.0;
        const j2 = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_Y))) / 127.0;

        // just do a simple tank drive
        ldr = @intFromFloat(j1 * 12000);
        rdr = @intFromFloat(j2 * 12000);
    }

    // check for toggling of the reverse toggle
    if (controller.get_digital_new_press(reverse_toggle)) {
        reverse.* = !reverse.*;
        // if toggled on, vibrate long, else short
        _ = if (reverse.*)
            pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, "-")
        else
            pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, ".");
    }

    // check for reverse driving toggle
    if (reverse.*) {
        // swap then negate
        const tmp = ldr;
        ldr = -rdr;
        rdr = -tmp;
    }

    // drive the drivetrain
    driveLeft(ldr, port_buffer);
    driveRight(rdr, port_buffer);
}

/// Drivetrain default configs (port is negative for reversed)
pub fn drivetrainMotor(comptime mport: comptime_int) Motor {
    return Motor{
        .port = mport,
        .gearset = pros.motors.E_MOTOR_GEAR_BLUE,
        .encoder_units = pros.motors.E_MOTOR_ENCODER_DEGREES,
    };
}

// Initializes the drivetrain (MUST BE RUN AT PROGRAM INIT)
pub fn init() void {
    drivetrain_motors.l1.init();
    drivetrain_motors.l2.init();
    drivetrain_motors.l3.init();
    drivetrain_motors.r1.init();
    drivetrain_motors.r2.init();
    drivetrain_motors.r3.init();
}

/// Returns the desired left and right drive velocities based on the controller.
/// For 'Toggle Arcade'
pub fn toggleArcade() struct { i32, i32 } {
    // gets the normalized x and y from the left joystick
    var x: f64 = undefined;
    var y = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127.0;

    // if split arcade, then split
    if (comptime options.split_arcade)
        x = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_X))) / 127.0
    else
        x = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_X))) / 127.0;

    // apply the rotation and movement multipliers
    x *= turn_multiplier;
    y *= drive_multiplier;

    // if R1 is hit, lock to rotation
    if (controller.get_digital(pros.misc.E_CONTROLLER_DIGITAL_Y)) {
        y = 0;
        // turning should be 'slower' when locking to rotation
        x *= drive_multiplier; // turning slowdown should be proportional to the movement speedup
    }

    // if R2 is hit, do NOT lock to movement
    if (controller.get_digital(pros.misc.E_CONTROLLER_DIGITAL_A)) {
        // movement should be 'faster' when locking to movement
        y /= drive_multiplier; // to undo the speed multiplier
    }

    // rest is just normal arcade drive
    return arcadeDrive(x, y);
}

/// Converts -1..=1 x & y values into left & right drive voltages
pub fn arcadeDrive(x: f64, y: f64) struct { i32, i32 } {
    const ldr = std.math.clamp(y + x, -1, 1);
    const rdr = std.math.clamp(y - x, -1, 1);

    return .{ @intFromFloat(ldr * 12000), @intFromFloat(rdr * 12000) };
}

/// Drives the drivetrain side based upon the input voltage, reports any motor
/// disconnects to the port buffer
pub fn driveLeft(voltage: i32, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.l1.setVoltage(voltage, port_buffer);
    drivetrain_motors.l2.setVoltage(voltage, port_buffer);
    drivetrain_motors.l3.setVoltage(voltage, port_buffer);
}

/// Drives the drivetrain side based upon the input voltage
/// 
/// Disconnect buffer is a buffer of disconnected motor ports, 0s are ignored
pub fn driveRight(voltage: i32, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.r1.setVoltage(voltage, port_buffer);
    drivetrain_motors.r2.setVoltage(voltage, port_buffer);
    drivetrain_motors.r3.setVoltage(voltage, port_buffer);
}
