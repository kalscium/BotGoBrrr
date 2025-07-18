//! Defines the driver-control routine

const std = @import("std");

const options = @import("options");
const pros = @import("pros");

const def = @import("def.zig");
const drive = @import("drive.zig");
const odom = @import("odom.zig");
const port = @import("port.zig");
const logging = @import("logging.zig");
const tower = @import("tower.zig");

/// The delay in ms, between each tick cycle
const cycle_delay = 50;

/// The path to the opcontrol port buffers file
const port_buffer_path = "/usd/opctrl_port_buffers.bin";

/// The path to the recorded coords file
const recrded_coords_path = "/usd/recrded_coords.txt";

/// The path to the CSV temperatures
const drive_temp_path = "/usd/opctrl_temps.csv";

/// The path to the CSV coordinates of the robot (path taken)
const coords_path = "/usd/opctrl_coords.csv";

/// The path to the CSV coordinates of the robot (path taken)
const velocities_path = "/usd/opctrl_velocities.csv";

/// The path to the CSV battery precentages (of the battery & controller)
const battery_path = "/usd/opctrl_battery.csv";

/// The path to the CSV benchmark
const bench_path = "/usd/opctrl_bench.csv";

/// Gets called during the driver-control period
pub fn opcontrol() callconv(.C) void {
    _ = pros.printf("hello, world from opcontrol!\n");

    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer logging.closeFile(port_buffer_file);

    // open the recorded coords file
    const recrded_coords_file = pros.fopen(recrded_coords_path, "w");
    defer logging.closeFile(recrded_coords_file);

    // open the drive motor temperature file
    const drive_temp_file = pros.fopen(drive_temp_path, "w");
    defer logging.closeFile(drive_temp_file);
    logging.writeHeader(drive_temp_file, logging.csv_header_temp);

    // open the odom coordinates file
    const coords_file = pros.fopen(coords_path, "w");
    defer logging.closeFile(coords_file);
    logging.writeHeader(coords_file, logging.csv_header_coords);

    // open the odom velocities file
    const velocities_file = pros.fopen(velocities_path, "w");
    defer logging.closeFile(velocities_file);
    logging.writeHeader(velocities_file, logging.csv_header_velocity);

    // open the battery percentage file
    const battery_file = pros.fopen(battery_path, "w");
    defer logging.closeFile(battery_file);
    logging.writeHeader(battery_file, logging.csv_header_battery);

    // open the benchmark file
    const bench_file = pros.fopen(bench_path, "w");
    defer logging.closeFile(bench_file);
    logging.writeHeader(bench_file, logging.csv_header_bench);

    // main loop state variables
    var now = pros.rtos.millis();
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);
    var drive_reversed = false;
    var cycles: u32 = 0;

    // main loop
    while (true) : (cycles += 1) {
    const compute_start_time = pros.rtos.millis();

    // update odom
    odom_state.update(&port_buffer);

    // update the drivetrain
    drive.controllerUpdate(&drive_reversed, &port_buffer);
    // update the tower
    tower.controllerUpdate(&port_buffer);
    // update odom controls
    odom_state.controllerUpdate(recrded_coords_file);

    const logging_start_time = pros.rtos.millis();

    // log the battery every 50 cycles
    if (cycles / 50 == 0)
        logging.battery(now, battery_file);

    // log the temperature every 32 cycles
    if (cycles / 32 == 0)
        logging.temp(now, drive_temp_file);

    // log odom coordinates every 16 cycles
    if (cycles / 16 == 0)
        logging.coords(coords_file, odom_state);

    // log odom velocities every tick
    if (comptime options.log_velocity)
        logging.velocity(velocities_file, odom_state);

    // write the port buffer to the port_buffer file
    if (port_buffer_file) |file|
        _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);

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

    const logging_end_time = pros.rtos.millis();

    // log the benchmark (every tick)
    if (comptime options.benchmark)
        logging.benchmark(bench_file, logging_start_time - compute_start_time, logging_end_time - logging_start_time, logging_end_time-compute_start_time);

    // wait for the next cycle
    pros.rtos.task_delay_until(&now, cycle_delay);

    }
}
