//! Defines the driver-control routine

const pros = @import("pros");
const def = @import("def.zig");
const drive = @import("drive.zig");
const port = @import("port.zig");
const odom = @import("odom.zig");

/// The delay in ms, between each tick cycle
const tick_delay = 50;

/// The path to the opcontrol port buffers file
const port_buffer_path = "/usd/opctrl_port_buffers.bin";

/// Gets called during the driver-control period
export fn opcontrol() callconv(.C) void {
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    // main loop state variables
    var now = pros.rtos.millis();
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    // main loop
    while (true) {

    // get the normalized main joystick values
    const j1 = .{
        .x = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.left_x)))) / 63.5,
        .y = @as(f32, @floatFromInt(pros.misc.controller_get_analog(@intFromEnum(def.Controller.master), @intFromEnum(def.ControllerAnalog.left_y)))) / 63.5,
    };
    
    // pass it through arcade drive to get left & right voltages
    const arcade = drive.arcadeDrive(j1.x, j1.y);
    const ldr: i32 = @intFromFloat(arcade[0]);
    const rdr: i32 = @intFromFloat(arcade[1]);

    // drive the drivetrain
    drive.driveLeft(ldr, &port_buffer);
    drive.driveRight(rdr, &port_buffer);

    // update odom
    odom.updateOdom(&odom_state, &port_buffer);

    // logs
    if (port_buffer_file) |file| _ = pros.fwrite(@ptrCast(&port_buffer), @sizeOf(port.PortBuffer), 1, file);

    // wait for the next cycle
    pros.rtos.task_delay_until(&now, tick_delay);

    }
}
