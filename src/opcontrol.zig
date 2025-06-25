//! Defines the driver-control routine

const std = @import("std");

const options = @import("options");
const pros = @import("pros");

const def = @import("def.zig");
const drive = @import("drive.zig");
const odom = @import("odom.zig");
const port = @import("port.zig");
const logging = @import("logging.zig");

/// The delay in ms, between each tick cycle
const tick_delay = 50;

/// The path to the opcontrol port buffers file
const port_buffer_path = "/usd/opctrl_port_buffers.bin";

/// The path to the recorded cords file
const recrded_coords_path = "/usd/recrded_coords.txt";

/// The path to the CSV drive motor temperatures
const drive_temp_path = "/usd/opctrl_drive_temp.csv";

/// The path to the CSV coordinates of the robot (path taken)
const coords_path = "/usd/opctrl_coords.csv";

/// The path to the CSV coordinates of the robot (path taken)
const velocities_path = "/usd/opctrl_velocities.csv";

/// Gets called during the driver-control period
pub fn opcontrol() callconv(.C) void {
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    // open the recorded coords file
    const recrded_coords_file = pros.fopen(recrded_coords_path, "w");
    defer if (recrded_coords_file) |file| {
        _ = pros.fclose(file);
    };

    // open the drive motor temperature file
    const drive_temp_file = pros.fopen(drive_temp_path, "w");
    defer if (drive_temp_file) |file| {
        _ = pros.fclose(file);
    };
    if (drive_temp_file) |file| {
        _ = pros.fprintf(file, logging.csv_header_temp);
    }
    // amount of times the drive motor temperatures have been logged
    var logged_drive_temp: u16 = 0;

    // open the odom coordinates file
    const coords_file = pros.fopen(coords_path, "w");
    defer if (coords_file) |file| {
        _ = pros.fclose(file);
    };
    if (coords_file) |file| {
        _ = pros.fprintf(file, logging.csv_header_coords);
    }
    // amount of times the odom coords have been logged
    var logged_coords: u16 = 0;

    // open the odom coordinates file
    const velocities_file = pros.fopen(velocities_path, "w");
    defer if (velocities_file) |file| {
        _ = pros.fclose(file);
    };
    if (velocities_file) |file| {
        _ = pros.fprintf(file, logging.csv_header_velocity);
    }

    // main loop state variables
    var now = pros.rtos.millis();
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    // main loop
    while (true) {

    // update odom
    odom_state.update(&port_buffer);
    
    // hopefully gets set by one of the options
    var ldr: i32 = 0;
    var rdr: i32 = 0;

    if (options.arcade) {
        // get the normalized main joystick values
        const jx = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.left_x)))) / 127;
        const jy = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.left_y)))) / 127;
        ldr, rdr = drive.arcadeDrive(jx, jy);
    } else if (options.split_arcade) {
        // get the normalized main joystick values
        const j1 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.left_x)))) / 127;
        const j2 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.right_y)))) / 127;
        ldr, rdr = drive.arcadeDrive(j1, j2);
    } else {
        // get the normalized main joystick values
        const j1 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.left_y)))) / 127;
        const j2 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.right_y)))) / 127;

        // just do a simple tank drive
        ldr = @intFromFloat(j1 * 12000);
        rdr = @intFromFloat(j2 * 12000);
    }

    // drive the drivetrain
    drive.driveLeft(ldr, &port_buffer);
    drive.driveRight(rdr, &port_buffer);

    // check for the 'record position' button press, print to both file & stdout
    if (pros.misc.controller_get_digital_new_press(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerDigital.x)) == 1) {
        // super compact and efficient binary files are cool and all but they
        // just aren't worth it for something like this where it'd be written
        // to like 8 times at most instead of EVERY TICK
        _ = pros.printf("Recorded Coord at: .{ %f, %f }\n", odom_state.coord[0], odom_state.coord[1]);
        if (recrded_coords_file) |file|
            _ = pros.fprintf(file, "Recorded Coord at: .{ %f, %f }\n", odom_state.coord[0], odom_state.coord[1]);
    }

    // log the temperature every 320ms
    if (logged_drive_temp < now / 320) {
        logged_drive_temp += 1;
        if (drive_temp_file) |file|
            logging.temp(now, file);
    }

    // log odom coordinates every 160ms
    if (logged_coords < now / 160) {
        logged_coords += 1;
        if (coords_file) |file|
            logging.coords(file, odom_state);
    }

    // log odom velocities every tick
    if (comptime options.log_velocity)
    if (velocities_file) |file|
        logging.velocity(file, odom_state);

    // write the port buffer to the port_buffer file
    if (port_buffer_file)
        |file| _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);

    // print the port buffer to stdout
    if (!(pros.misc.competition_is_connected() > 0)) {
        // print the port buffer if there are disconnects
        if (@as(u24, @bitCast(port_buffer)) != 0xFFFFFF) {
            _ = pros.printf("Disconnected Ports:");
            inline for (1..22) |iport| {
                const field = std.fmt.comptimePrint("port_{}", .{iport});
                if (!@field(port_buffer, field))
                    _ = pros.printf(" %d", iport);
            } _ = pros.printf("\n");
        }
    }

    // wait for the next cycle
    pros.rtos.task_delay_until(&now, tick_delay);

    }
}
