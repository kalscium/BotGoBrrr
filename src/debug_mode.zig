//! A special opcontrol-mode that can be enabled to test, debug, showcase and plan autonomous

const std = @import("std");
const pros = @import("pros");
const logging = @import("logging.zig");
const port = @import("port.zig");
const odom = @import("odom.zig");
const pure_pursuit = @import("pure_pursuit.zig");
const auton = @import("autonomous.zig");
const drive = @import("drive.zig");
const controller = @import("controller.zig");
const vector = @import("vector.zig");

/// Tank drive velocity (from 0..=1)
const tank_vel: f64 = 0.2;

/// The path to the debug-mode log file
const log_path = "/usd/dbgmode.log";

const TunedParameter = enum(i8) {
    search_radius = 0,
    proportional = 1,
    lookahead_window = 2,

    pub fn cycle(self: *TunedParameter, amount: i8) void {
        const raw = @as(i8, @intFromEnum(self.*)) + amount;
        const wrapped = @mod(raw, 3);
        self.* = @enumFromInt(wrapped);
    }

    pub fn set(self: TunedParameter, params: *pure_pursuit.Parameters, val: f64) void {
        switch (self) {
            .search_radius => params.search_radius = val,
            .proportional => params.kp = val,
            .lookahead_window => params.lookahead_window = val,
        }
    }

    pub fn get(self: TunedParameter, params: pure_pursuit.Parameters) f64 {
        return switch (self) {
            .search_radius => params.search_radius,
            .proportional => params.kp,
            .lookahead_window => params.lookahead_window,
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
    var params: pure_pursuit.Parameters = auton.pure_pursuit_params;
    var incr_order_of_mag: f64 = 0; // the order of magnitude of the parameter increments
    var tuned_param = TunedParameter.search_radius; // the current auton paramter getting tuned
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
            drive.driveLeft(ldr, &port_buffer);
            drive.driveRight(rdr, &port_buffer);
        } else {
            // run auton based upon the path stack

            // otherwise calculate pure pursuit and follow it
            // calculate the robot's predicted location and base all future calculations off of it
            const predicted = pure_pursuit.predictCoordYaw(odom_state, params.lookahead_window);
            const path_seg_start = pure_pursuit.pickPathPoints(predicted.coord, params.search_radius, path_stack.slice(), &last_end);
            const goal_point = pure_pursuit.interpolateGoal(predicted.coord, params.search_radius, path_seg_start, path_stack.slice()[last_end]);
            const ratios = pure_pursuit.followArc(predicted.coord, goal_point, predicted.yaw);
            const speed = pure_pursuit.speedController(predicted.coord, goal_point, params);

            const ldr, const rdr = ratios * @as(@Vector(2, f64), @splat(speed));
            drive.driveLeft(ldr, &port_buffer);
            drive.driveRight(rdr, &port_buffer);

            // if it's within precision, rumble and pause
            if (vector.calMag(f64, path_stack.slice()[last_end] - odom_state.coord) < auton.precision_mm) {
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
                _ = pros.fprintf(file, "pure pursuit parameters:\n");
                inline for (@typeInfo(pure_pursuit.Parameters).@"struct".fields) |field|
                    _ = pros.fprintf(file, "  * %s: %lf\n", @as([*:0]const c_char, @ptrCast(field.name)), @field(params, field.name));

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
            tuned_param.set(&params, tuned_param.get(params) * 2.0)
        else if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_L1))
            tuned_param.set(&params, tuned_param.get(params) / 2.0);

        // left & right arrows increase and decrease the order of magnitude
        // (moves the decimal point)
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_LEFT))
            incr_order_of_mag += 1
        else if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_RIGHT))
            incr_order_of_mag -= 1;

        // R2 increments, R1 decrements (by the order of magnitude)
        if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_R2))
            tuned_param.set(&params, tuned_param.get(params) + std.math.pow(f64, 10.0, incr_order_of_mag))
        else if (controller.get_digital_new_press(pros.misc.E_CONTROLLER_DIGITAL_R1))
            tuned_param.set(&params, tuned_param.get(params) - std.math.pow(f64, 10.0, incr_order_of_mag));

        // display the parameter tuning menu on the controller
        if (cycles % 20 == 0) { // every 150ms
            // reports the current auton parameter getting tuned
            // list goes as follows:
            //   * search_radius
            //   * proportional
            //   * lookahead_win
            //   * 180degturnmul
            //   * precise_thres
            const label = switch (tuned_param) {
                .search_radius    => "search_radius",
                .proportional     => "proportional",
                .lookahead_window => "lookahead_win",
            };
            _ = pros.misc.controller_print(pros.misc.E_CONTROLLER_MASTER, 0, 0, ">%s", @as([*:0]const c_char, @ptrCast(label)));
        } else if (cycles % 20 == 5) {
            // reports the current order of magnitude of the increments
            _ = pros.misc.controller_print(pros.misc.E_CONTROLLER_MASTER, 1, 0, "10^%lf = %lf", incr_order_of_mag, std.math.pow(f64, 10, incr_order_of_mag));
        } else if (cycles % 20 == 10) {
            // reports the current value of the tuned parameter
            _ = pros.misc.controller_print(pros.misc.E_CONTROLLER_MASTER, 2, 0, "$ %lf", tuned_param.get(params));
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
