//! Defines the driver-control routine

const std = @import("std");
const pros = @import("pros");
const def = @import("def.zig");
const drive = @import("drive.zig");
const port = @import("port.zig");
const odom = @import("odom.zig");

/// The delay in ms, between each tick cycle
const tick_delay = 50;

/// The path to the opcontrol port buffers file
const port_buffer_path = "/usd/opctrl_port_buffers.bin";

/// The path to the recorded cords file
const recrded_coords_path = "/usd/recrded_coords.txt";

/// Gets called during the driver-control period
export fn opcontrol() callconv(.C) void {
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

    // main loop state variables
    var now = pros.rtos.millis();
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    // main loop
    while (true) {

    // update odom
    odom.updateOdom(&odom_state, &port_buffer);

    // get the normalized main joystick values
    const j1 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.left_y)))) / 63.5;
    const j2 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.right_y)))) / 63.5;
    
    // just do a simple tank drive
    const ldr: i32 = @intFromFloat(j1 * 12000);
    const rdr: i32 = @intFromFloat(j2 * 12000);

    // drive the drivetrain
    drive.driveLeft(ldr, &port_buffer);
    drive.driveRight(rdr, &port_buffer);

    // check for the 'record position' button press, print to both file & stdout
    if (pros.misc.controller_get_digital(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerDigital.x)) > 0) {
        // super compact and efficient binary files are cool and all but they
        // just aren't worth it for something like this where it'd be written
        // to like 8 times at most instead of EVERY TICK
        if (!(pros.misc.competition_is_connected() > 0))
            _ = pros.printf("Recorded Coord at: .{ %f, %f }\n", odom_state.coord[0], odom_state.coord[1]);
        if (recrded_coords_file) |file|
            _ = pros.fprintf(file, "Recorded Coord at: .{ %f, %f }\n", odom_state.coord[0], odom_state.coord[1]);
    }

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
