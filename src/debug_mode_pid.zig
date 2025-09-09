//! A special opcontrol-mode that can be enabled to test, debug, showcase and plan autonomous

const std = @import("std");
const pros = @import("pros");
const logging = @import("logging.zig");
const port = @import("port.zig");
const odom = @import("odom.zig");
const pid = @import("pid.zig");
const auton = @import("autonomous.zig");
const drive = @import("drive.zig");
const controller = @import("controller.zig");
const vector = @import("vector.zig");

/// Tank drive velocity (from 0..=1)
const tank_vel: f64 = 0.2;

/// The path to the debug-mode log file
const log_path = "/usd/dbgmode_pid.log";

const TunedParameter = enum(i8) {
    yaw_kp = 0,
    yaw_ki = 1,
    yaw_kd = 2,
    mov_kp = 3,
    mov_ki = 4,
    mov_kd = 5,

    pub fn cycle(self: *TunedParameter, amount: i8) void {
        const raw = @as(i8, @intFromEnum(self.*)) + amount;
        const wrapped = @mod(raw, 3);
        self.* = @enumFromInt(wrapped);
    }

    pub fn set(self: TunedParameter, yaw_param: *pid.Param, mov_param: *pid.Param, val: f64) void {
        switch (self) {
            .yaw_kp => yaw_param.kp = val,
            .yaw_ki => yaw_param.ki = val,
            .yaw_kd => yaw_param.kd = val,
            .mov_kp => mov_param.kp = val,
            .mov_ki => mov_param.ki = val,
            .mov_kd => mov_param.kd = val,
        }
    }

    pub fn get(self: TunedParameter, yaw_param: pid.Param, mov_param: pid.Param) f64 {
        return switch (self) {
            .yaw_kp => yaw_param.kp,
            .yaw_ki => yaw_param.ki,
            .yaw_kd => yaw_param.kd,
            .mov_kp => mov_param.kp,
            .mov_ki => mov_param.ki,
            .mov_kd => mov_param.kd,
        };
    }
};

/// The entry function for the debug mode runtime
pub fn entry() void {
    // open the log file
    const log_file = pros.fopen(log_path, "w");
    defer logging.closeFile(log_file);

    // main loop state variables
    var now = pros.rtos.millis();
    var cycles: u32 = 0;
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connnected/working initially
    var odom_state = odom.State.init(&port_buffer);
    var auton_paused = true; // if the auton is currently paused (not running)

    // the path point stack, last_end & tuned pure pursuit parameters
    var path_stack = std.BoundedArray(odom.Coord, 32).init(0) catch unreachable; // 32 is a reasonable amount
    var last_end: usize = 0;
    var yaw_params = auton.yaw_pid_param;
    var mov_params = auton.mov_pid_param;
    var incr_order_of_mag: f64 = 0; // the order of magnitude of the parameter increments
    var tuned_param = TunedParameter.yaw_kp; // the current auton paramter getting tuned
    var rumble: enum { none, short, long } = .none; // concurrent queuing for the controller rumble (50ms speed limit)

    // main loop
    while (true) : (cycles += 1) {
        // update odom
        odom_state.update(&port_buffer);

        // if X is hit, then toggle the start/stop of auton
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_X))
            auton_paused = !auton_paused;

        if (auton_paused) {
            // update the drivetrain (tank drive)
            const ldr = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127.0 * tank_vel;
            const rdr = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_Y))) / 127.0 * tank_vel;
            drive.driveVolt(@intFromFloat(ldr * 12000), @intFromFloat(rdr * 12000), &port_buffer);
        } else {
            // run auton based upon the path stack

            // otherwise calculate pure pursuit and follow it
            // calculate the robot's predicted location and base all future calculations off of it
            for (path_stack.slice()) |point| {
                // calculate the point coord relative to the current coord and rotate and move to it
                const rel_point = point - odom_state.coord;
                const angle = vector.calDir(f64, rel_point);
                const distance = vector.calMag(f64, rel_point);

                pid.rotateDeg(std.math.radiansToDegrees(angle), &odom_state, &port_buffer);
                pid.move(distance, &odom_state, &port_buffer);
            }

            // if it's within precision, rumble and pause
            if (vector.calMag(f64, path_stack.slice()[path_stack.len-1] - odom_state.coord) < auton.precision_mm) {
                rumble = .long;
                auton_paused = !auton_paused;
            }
        }

        // if A is hit, push to the stack (checking for overflow)
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_A)) {
            if (path_stack.append(odom_state.coord)) {
                rumble = .short;
            } else |_| {
                // long rumble controller on overflow
                rumble = .long;
            }
        }

        // if B is hit, pop from the stack (checking for underflow)
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_B)) {
            if (path_stack.pop() == null) {
                // long rumble controller on underflow
                rumble = .long;
            } else {
                rumble = .short;
            }
        }

        // if Y is hit, log then reset all values
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_Y)) {
            // long rumble
            rumble = .long;

            // log the info
            if (log_file) |file| {
                _ = pros.fprintf(file, "=== DEBUG MODE LOGGED VALUES ===\n");
                // print the path point stack
                _ = pros.fprintf(file, "path_point_stack: .{ ");
                for (path_stack.slice()) |point|
                    _ = pros.fprintf(file, ".{ %lf, %lf }, ", point[0], point[1]);
                _ = pros.fprintf(file, "}\n");

                // print the parameters
                _ = pros.fprintf(file, "yaw PID parameters:\n");
                inline for (@typeInfo(pid.Param).@"struct".fields) |field|
                    _ = pros.fprintf(file, "  * %s: %lf\n", @as([*:0]const c_char, @ptrCast(field.name)), @field(yaw_params, field.name));
                _ = pros.fprintf(file, "mov PID parameters:\n");
                inline for (@typeInfo(pid.Param).@"struct".fields) |field|
                    _ = pros.fprintf(file, "  * %s: %lf\n", @as([*:0]const c_char, @ptrCast(field.name)), @field(mov_params, field.name));

                // print the port buffer if there are disconnects
                if (@as(u24, @bitCast(port_buffer)) != 0xFFFFFF) {
                    _ = pros.fprintf(file, "Disconnected Ports:");
                    inline for (1..22) |iport| {
                        const field = std.fmt.comptimePrint("port_{}", .{iport});
                        if (!@field(port_buffer, field))
                            _ = pros.fprintf(file, " %d", iport);
                    } _ = pros.fprintf(file, "\n\n");
                }
            }

            // reset everything that's not the path or parameters
            odom_state = odom.State.init(&port_buffer);
            _ = pros.imu.imu_tare_yaw(odom.imu_port);
            _ = pros.rotation.rotation_reset_position(odom.rotation_port_vertical);
            _ = pros.rotation.rotation_reset_position(odom.rotation_port_lateral);
            last_end = 0;
        }

        // the tuned parameters are cycled through the up & down arrows
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_UP))
            tuned_param.cycle(-1)
        else if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_DOWN))
            tuned_param.cycle(1);

        // L2 doubles the parameter, L1 halves it
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_L2))
            tuned_param.set(&yaw_params, &mov_params, tuned_param.get(yaw_params, mov_params) * 2.0)
        else if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_L1))
            tuned_param.set(&yaw_params, &mov_params, tuned_param.get(yaw_params, mov_params) / 2.0);

        // left & right arrows increase and decrease the order of magnitude
        // (moves the decimal point)
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_LEFT))
            incr_order_of_mag += 1
        else if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_RIGHT))
            incr_order_of_mag -= 1;

        // R2 increments, R1 decrements (by the order of magnitude)
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_R2))
            tuned_param.set(&yaw_params, &mov_params, tuned_param.get(yaw_params, mov_params) + std.math.pow(f64, 10.0, incr_order_of_mag))
        else if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_R1))
            tuned_param.set(&yaw_params, &mov_params, tuned_param.get(yaw_params, mov_params) - std.math.pow(f64, 10.0, incr_order_of_mag));

        // display the parameter tuning menu on the controller
        if (cycles % 20 == 0) { // every 150ms
            // reports the current auton parameter getting tuned
            const label = switch (tuned_param) {
                .yaw_kp => "yaw_kp",
                .yaw_ki => "yaw_ki",
                .yaw_kd => "yaw_kd",
                .mov_kp => "mov_kp",
                .mov_ki => "mov_ki",
                .mov_kd => "mov_kd",
            };
            _ = pros.misc.controller_print(pros.misc.E_CONTROLLER_MASTER, 0, 0, ">%s", @as([*:0]const c_char, @ptrCast(label)));
        } else if (cycles % 20 == 5) {
            // reports the current order of magnitude of the increments
            _ = pros.misc.controller_print(pros.misc.E_CONTROLLER_MASTER, 1, 0, "10^%lf = %lf", incr_order_of_mag, std.math.pow(f64, 10, incr_order_of_mag));
        } else if (cycles % 20 == 10) {
            // reports the current value of the tuned parameter
            _ = pros.misc.controller_print(pros.misc.E_CONTROLLER_MASTER, 2, 0, "$ %lf", tuned_param.get(yaw_params, mov_params));
        } else if (cycles % 20 == 15) {
            // rumbles if the controller needs rumbling
            if (rumble == .short)
                _ = pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, ".")
            else if (rumble == .long) 
                _ = pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, "-");
            rumble = .none;
        }

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}
