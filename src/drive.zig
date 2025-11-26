//! Functions for driving the robot (calculations & drivetrain)

const std = @import("std");
const Motor = @import("Motor.zig");
const port = @import("port.zig");
const pros = @import("pros");
const def = @import("def.zig");
const options = @import("options");
const controller = @import("controller.zig");
const pid = @import("pid.zig");
const opcontrol = @import("opcontrol.zig");
const auton = @import("autonomous.zig");
const vector = @import("vector.zig");
const odom = @import("odom.zig");

/// For turning to absolute angles (from start)
pub const absolute_turn_held: c_int = pros.misc.E_CONTROLLER_DIGITAL_DOWN;

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
    pub const l1 = drivetrainMotor(-11);
    pub const l2 = drivetrainMotor(-2);
    pub const l3 = drivetrainMotor(12);
    pub const r1 = drivetrainMotor(10);
    pub const r2 = drivetrainMotor(9);
    pub const r3 = drivetrainMotor(-14);
};

/// The multiplier applied to the robot's movement speed normally
pub const drive_multiplier = 1.0;
/// The multiplier applied to the robot's turning speed normally
pub const turn_multiplier = 1.0;

pub const DriveState = struct {
    yaw_pid: pid.State = .{},
};

/// Daniel's magic equation.
/// 
/// Logarithmic at the start to overcome dead-zones, and a ~0.7 flattened linear
/// for precise movement to the end.
/// 
/// https://www.desmos.com/calculator/xj1enleneb
pub fn dme(x: f64) f64 {
    // don't ask what anything means, just that it works

    const a = 4.0;
    const b = 0.6;
    const c = b * @log(2.0 * a);

    const abs_x = @abs(x); // negatives treated same as positives
    const sgn_x = std.math.sign(x);

    return @exp((@log(a) - @sqrt(@log(a)*@log(a) - 4.0 * c * @log(abs_x)))/2.0) * sgn_x;
}

/// Reads the controller and updates the drivetrain accordingly based upon the
/// enabled build options
/// 
/// Also takes in whether the robot is currently driving backwards (in reverse)
/// 
/// Updates the port buffer on any motor disconnects
pub fn controllerUpdate(drive_state: *DriveState, port_buffer: *port.PortBuffer) void {
    if (controller.get_digital(absolute_turn_held)) {
        const ldr, const rdr = absTurn(drive_state, port_buffer);
        driveVel(ldr, rdr, port_buffer);
    } else {
        // gets the normalized x and y from the right and left joystick passed through spite and scaled with the multipliers
        var x = dme(@as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_X))) / 127.0) * turn_multiplier;
        var y = dme(@as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127.0) * drive_multiplier;
    
        // to stop the driver from maxing out joystick (make this an option)
        if (comptime options.prac_driv)
        if (x == 1.0 or y == 1.0) {
            x = 0.0;
            y = 0.0;
        };

        // otherwise rest is just normal arcade drive converted to millivolts
        // const ldr, const rdr = @as(@Vector(2, i32), @intFromFloat(arcadeDrive(x, y) * @as(@Vector(2, f64), @splat(12000.0))));
        // const ldr, const rdr = @as(@Vector(2, i32), @intFromFloat(arcadeDriveMaxDesat(x * 1.2, y) * @as(@Vector(2, f64), @splat(12000.0))));
        const ldr, const rdr = @as(@Vector(2, i32), @intFromFloat(arcadeDriveCurveDesat(x * 1.2, y) * @as(@Vector(2, f64), @splat(12000.0))));
        driveVolt(ldr, rdr, port_buffer);
    }
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

/// Turns to an absolute angle (from IMU) relative to it's starting angle
pub fn absTurn(drive_state: *DriveState, port_buffer: *port.PortBuffer) struct { f64, f64 } {
    // gets the normalized y from the left joystick, and the x and y from the right joystick
    const y = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127.0;
    const j2x = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_X))) / 127.0;
    const j2y = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_Y))) / 127.0;

    // get the 'x' value for the turn from the PID (if the right joystick is being used)
    var x: f64 = 0;
    if (@abs(j2x) > 0 or @abs(j2y) > 0) {
        const desired = vector.calDir(f64, .{ j2x, j2y }); // calculate the desired angle
        const yaw = odom.getYaw(port_buffer) orelse 0;
        const err = odom.minimalAngleDiff(yaw, desired);
        x = drive_state.yaw_pid.update(auton.yaw_pid_param, err, opcontrol.cycle_delay);
    }

    // pass it through arcade drive
    return .{
        std.math.clamp(y + x, -1, 1),
        std.math.clamp(y - x, -1, 1),
    };
}

/// Converts -1..=1 x & y values into left & right drive velocities
pub fn arcadeDrive(x: f64, y: f64) @Vector(2, f64) {
    // apply the rotation and movement multipliers
    const n_x = x * turn_multiplier;
    const n_y = y * drive_multiplier;

    const ldr = std.math.clamp(n_y + n_x, -1, 1);
    const rdr = std.math.clamp(n_y - n_x, -1, 1);

    return .{ ldr, rdr };
}

/// Converts -1..=1 x & y values into desaturated left & right drive velocities (my algorithm)
pub fn arcadeDriveMyDesat(steer: f64, throttle: f64) @Vector(2, f64) {
    const ldr = throttle + steer;
    const rdr = throttle - steer;
    // const max = @max(@abs(ldr), @abs(rdr));
    const max = @sqrt(ldr*ldr + rdr*rdr);

    // get the overflows
    const lof = if (@abs(ldr) > 1) (ldr - std.math.sign(ldr)) / max else 0;
    const rof = if (@abs(rdr) > 1) (rdr - std.math.sign(rdr)) / max else 0;

    // if one of them is overflowing then desat relative to it
    return .{ std.math.clamp(ldr, -1, 1) - rof, std.math.clamp(rdr, -1, 1) - lof };
}

/// Converts -1..=1 x & y values into desaturated left & right drive velocities (my algorithm)
pub fn arcadeDriveCurveDesat(steer: f64, throttle: f64) @Vector(2, f64) {
    const ldr = throttle + steer * @min((1.6 - @abs(throttle)), 1.0);
    const rdr = throttle - steer * @min((1.6 - @abs(throttle)), 1.0);

    return .{ ldr, rdr };
}

/// Converts -1..=1 x & y values into desaturated left & right drive velocities (diamond shape)
pub fn arcadeDriveMaxDesat(steer: f64, throttle: f64) @Vector(2, f64) {
    const ldr = throttle + steer;
    const rdr = throttle - steer;
    const max = @max(@abs(ldr), @abs(rdr));

    // if one of them is overflowing then desat relative to it
    if (max <= 1)
        return .{ ldr, rdr }
    else
        return .{ ldr/max, rdr/max };
}

/// Converts -1..=1 x & y values into desaturated left & right drive velocities (circle shape)
pub fn arcadeDriveCircleDesat(steer: f64, throttle: f64) @Vector(2, f64) {
    const ldr = throttle + steer;
    const rdr = throttle - steer;
    const mag = @sqrt(ldr*ldr + rdr*rdr);

    // if one of them is overflowing then desat relative to it
    if (mag <= 1)
        return .{ ldr, rdr }
    else
        return .{ ldr/mag, rdr/mag };
}

/// reports any motor disconnects to the port buffer
pub fn driveVolt(ldr: i32, rdr: i32, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.l1.setVoltage(ldr, port_buffer);
    drivetrain_motors.l2.setVoltage(ldr, port_buffer);
    drivetrain_motors.l3.setVoltage(ldr, port_buffer);
    drivetrain_motors.r1.setVoltage(rdr, port_buffer);
    drivetrain_motors.r2.setVoltage(rdr, port_buffer);
    drivetrain_motors.r3.setVoltage(rdr, port_buffer);
}

/// Drives the drivetrain based upon the input velocities for left and right,
/// reports any motor disconnects to the port buffer
pub fn driveVel(ldr: f64, rdr: f64, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.l1.setVelocity(ldr, port_buffer);
    drivetrain_motors.l2.setVelocity(ldr, port_buffer);
    drivetrain_motors.l3.setVelocity(ldr, port_buffer);
    drivetrain_motors.r1.setVelocity(rdr, port_buffer);
    drivetrain_motors.r2.setVelocity(rdr, port_buffer);
    drivetrain_motors.r3.setVelocity(rdr, port_buffer);
}
