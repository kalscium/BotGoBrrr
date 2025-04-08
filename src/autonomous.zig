//! Defines the driver-control routine

const pros = @import("pros");
const port = @import("port.zig");
const odom = @import("odom.zig");
const vector = @import("vector.zig");

/// The delay in ms, between each 'cycle' of autonomous (the lower the moreprecise though less stable)
const tick_delay = 50;

/// The path to the autonomous port buffers file
const port_buffer_path = "/usd/auton_port_buffers.bin";

/// The path (an array of vector positions the robot must follow)
const ppath: [1]odom.Coord = .{odom.start_coord};

/// The 'precision' that the robot must achieve before moving onto the next path coordinate
const pprecision: f64 = 0;

export fn autonomous() callconv(.C) void {
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    // main loop state variables
    var now = pros.rtos.millis();
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    var ppath_idx: usize = 0; // for later, when pure-pursuit is implemented
    // for actions and states, instead of having loops within loops

    // main loop
    while (true) {
        // update odom
        odom.updateOdom(&odom_state, &port_buffer);
    
        // if the robot has not reached the current 'goal'
        if (@abs(vector.calMag(f64, odom_state.coord - ppath[ppath_idx])) > pprecision) {
            // toto: code to get the robot closer to the current goal
            continue;
        }

        // assume the robot has reached it's current goal

        // toto: put actions that depend on the location of the robot here
        switch (ppath_idx) {
            else => {},
        }

        // write the port buffer to the port_buffer file
        if (port_buffer_file) |file|
            _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);

        // either move onto the next goal or break the loop if finished
        if (ppath_idx == ppath.len) break;
        ppath_idx += 1;

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, tick_delay);
    }
}
