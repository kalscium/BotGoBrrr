//! A simple program to make the binary port_buffer logs human-readable

const std = @import("std");
const port = @import("src/port.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    const allocator = arena.allocator();

    // get the specified file path
    const args = try std.process.argsAlloc(allocator);
    defer allocator.free(args);
    if (args.len != 2)
        return error.InvalidArgs;
    const file_path = args[1];

    // open the file
    var file = try std.fs.cwd().openFile(file_path, .{ .mode = .read_only });
    defer file.close();

    // initialise the port buffer and an array pointer to it
    var port_buffer: port.PortBuffer = undefined;
    const port_buffer_ptr = @as([*]u8, @ptrCast(&port_buffer))[0..comptime @bitSizeOf(port.PortBuffer)/8];
    var total_port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume everything is connected

    while ((try file.readAll(port_buffer_ptr)) == comptime @bitSizeOf(port.PortBuffer)/8) {
        total_port_buffer = @bitCast(@as(u24, @bitCast(total_port_buffer)) & @as(u24, @bitCast(port_buffer)));
    }

    const stdout = std.io.getStdOut();

    // print total disconnects
    try std.fmt.format(stdout.writer(), "Total Port Disconnects:", .{});
    inline for (1..22) |iport| {
        const field = std.fmt.comptimePrint("port_{}", .{iport});
        if (!@field(total_port_buffer, field))
            try std.fmt.format(stdout.writer(), " {}", .{iport});
    } try std.fmt.format(stdout.writer(), "\n", .{});

    // print final disconnects
    try std.fmt.format(stdout.writer(), "Final Port Disconnects:", .{});
    inline for (1..22) |iport| {
        const field = std.fmt.comptimePrint("port_{}", .{iport});
        if (!@field(port_buffer, field))
            try std.fmt.format(stdout.writer(), " {}", .{iport});
    } try std.fmt.format(stdout.writer(), "\n", .{});
}
