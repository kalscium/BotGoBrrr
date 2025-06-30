//! Functions for driving the robot (calculations & drivetrain)

const std = @import("std");
const Motor = @import("Motor.zig");
const port = @import("port.zig");
const pros = @import("pros");
const def = @import("def.zig");
const options = @import("options");
const controller = @import("controller.zig");

/// Driving in reverse toggle button
pub const reverse_toggle: c_int = pros.misc.E_CONTROLLER_DIGITAL_X;

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
    pub const l1 = drivetrainMotor(-14);
    pub const l2 = drivetrainMotor(-8);
    pub const l3 = drivetrainMotor(9);
    pub const r1 = drivetrainMotor(1);
    pub const r2 = drivetrainMotor(10);
    pub const r3 = drivetrainMotor(-5);
};

/// Daniel's Magic Number for nice, smooth and exponential controls
/// 
/// *`a` is Daniel's Magic Number*
/// ```
/// $$
/// f(x) = ba^x - b
/// a = 1 + \frac{1}{b}
/// $$
/// ```
pub const DMN = struct {
    pub const a = 1 + @as(comptime_float, 1) / b;
    pub const b = 0.05;
};

/// The multiplier applied to the robot's turning
pub const turning_multiplier = 0.64;

/// Reads the controller and updates the drivetrain accordingly based upon the
/// enabled build options
/// 
/// Also takes in whether the robot is currently driving backwards (in reverse)
/// 
/// Updates the port buffer on any motor disconnects
pub fn controllerUpdate(reverse: *bool, port_buffer: *port.PortBuffer) void {
    // hopefully gets set by one of the options
    var ldr: f64 = 0;
    var rdr: f64 = 0;

    if (comptime options.arcade) {
        // get the normalized main joystick values
        const jx = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_X))) / 127;
        const jy = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127;
        ldr, rdr = userArcadeDrive(jx, jy);
    } else if (comptime options.toggle_arcade) {
        // get the normalized main joystick values
        var jx = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_X))) / 127;
        var jy = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127;

        // check for the toggles
        if (controller.get_digital(pros.misc.E_CONTROLLER_DIGITAL_R1))
            jy = 0;
        if (controller.get_digital(pros.misc.E_CONTROLLER_DIGITAL_R2))
            jx = 0;

        ldr, rdr = userArcadeDrive(jx, jy);
    } else if (comptime options.split_arcade) {
        // get the normalized main joystick values
        const j1 = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_X))) / 127;
        const j2 = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_Y))) / 127;
        ldr, rdr = userArcadeDrive(j1, j2);
    } else {
        // get the normalized main joystick values
        const j1 = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127;
        const j2 = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_Y))) / 127;

        // just do a simple tank drive
        ldr = j1;
        rdr = j2;
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

/// Passes a normalized value through daniel's algorithm to produce an exponential voltage
pub fn expDaniel(x: f64) f64 {
    return std.math.copysign(DMN.b * std.math.pow(f64, DMN.a, @abs(x)) - DMN.b, x);
}

test expDaniel {
    std.debug.assert(expDaniel(1) == 1);
    std.debug.assert(expDaniel(0) == 0);
    std.debug.assert(expDaniel(-1) == -1);
}

/// Converts user joystick x & y values into left & right drive velocities
pub fn userArcadeDrive(x: f64, y: f64) struct { f64, f64 } {
    var nx = x;
    var ny = y;

    // applies Daniel's Magic Number if enabled
    if (comptime options.DMN) {
        nx = expDaniel(x);
        ny = expDaniel(y);
    }

    // turning multipliers are applied after DMN, to preserve the full curve
    nx *= turning_multiplier;

    const ldr = std.math.clamp(ny + nx, -1, 1);
    const rdr = std.math.clamp(ny - nx, -1, 1);

    return .{ ldr, rdr };
}

/// Converts -1..=1 x & y values into left & right drive velocities
pub fn arcadeDrive(x: f64, y: f64) struct { f64, f64 } {
    var nx = x;
    var ny = y;

    // applies Daniel's Magic Number if enabled
    if (comptime options.DMN) {
        nx = expDaniel(x);
        ny = expDaniel(y);
    }

    // turning multipliers are applied after DMN, to preserve the full curve
    nx *= turning_multiplier;

    const ldr = std.math.clamp(y + x, -1, 1);
    const rdr = std.math.clamp(y - x, -1, 1);

    return .{ ldr, rdr };
}

/// Drives the drivetrain side based upon the input velocity, reports any motor
/// disconnects to the port buffer
pub fn driveLeft(velocity: f64, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.l1.setVelocity(velocity, port_buffer);
    drivetrain_motors.l2.setVelocity(velocity, port_buffer);
    drivetrain_motors.l3.setVelocity(velocity, port_buffer);
}

/// Drives the drivetrain side based upon the input velocity
/// 
/// Disconnect buffer is a buffer of disconnected motor ports, 0s are ignored
pub fn driveRight(velocity: f64, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.r1.setVelocity(velocity, port_buffer);
    drivetrain_motors.r2.setVelocity(velocity, port_buffer);
    drivetrain_motors.r3.setVelocity(velocity, port_buffer);
}
