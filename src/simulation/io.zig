//! Functions for input-output sandboxing & handling in the simulation

const std = @import("std");
const simulation = @import("../simulation.zig");

/// Returns a 'sandboxed' fs path owned by the caller
pub fn sandboxPath(path: [*:0]const u8) ![]const u8 {
    // if simulation-fs dir doesn't exist, then create it
    std.fs.cwd().makeDir("simulation-fs") catch {};

    return std.fmt.allocPrint(simulation.allocator, "simulation-fs/{s}", .{
        path["/usd/".len..] // strip the '/usd/' start
    });
}

/// Custom implementation of the fopen func
export fn fopen(filename: [*:0]const u8, mode: [*:0]const u8) ?*std.fs.File { // supposed to be ?*std.c.FILE but this is an evil hack to get things working
    // get the relative path
    const rel_path = sandboxPath(filename) catch return null;
    defer simulation.allocator.free(rel_path);

    // evil hack
    const file = simulation.allocator.create(std.fs.File) catch return null; // freed by fclose
    file.* = if (mode[0] == 'w')
        std.fs.cwd().createFile(rel_path, .{}) catch return null
    else
        std.fs.cwd().openFile(rel_path, .{}) catch return null;

    return file;
}

/// Custom implementation of the fprintf func
export fn fprintf(file: *std.fs.File, fmt: [*:0]const u8, ...) c_int {
    // get va_list and stdout
    var va_list = @cVaStart();
    defer @cVaEnd(&va_list);

    format(file.writer(), fmt, &va_list) catch {};

    return 0; // OK code
}

/// Custom implementation of the fclose func
export fn fclose(file: *std.fs.File) c_int { // should be *std.c.FILE, but this is an evil hack
    file.close(); // close the file
    simulation.allocator.destroy(file); // free the file ptr
    return 0;
}

/// Custom implementation of the printf func
export fn printf(fmt: [*:0]const u8, ...) callconv(.C) c_int {
    // get va_list and stdout
    var va_list = @cVaStart();
    defer @cVaEnd(&va_list);
    const stdout = std.io.getStdOut().writer();

    format(stdout, fmt, &va_list) catch {};

    return 0; // OK code
}

/// Custom implementation of the C string formatting
pub fn format(writer: anytype, fmt: [*:0]const u8, va_list: *std.builtin.VaList) !void {
    // start parsing and printing the fmt
    var parse_arg = false; // if it's currently parsing an arg
    var i: usize = 0;
    var char = fmt[i];
    while (fmt[i] != 0) : (i += 1) {
        char = fmt[i];

        if (parse_arg) {
            if (char == '%') // % is escaped
                try writer.writeByte(char)
            else if (char == 'l') // 'long' arguments (like %lf)
                continue // don't unset parse_arg
            else if (char == 'd' or char == 'i') // integers
                try std.fmt.format(writer, "{}", .{@cVaArg(va_list, c_int)})
            else if (char == 'f') // floating-point
                // for some reason only f64 produces valid values...
                try std.fmt.format(writer, "{d}", .{@cVaArg(va_list, f64)})
            else
                std.debug.panic("unimplemented fmt specifier '{c}'", .{char});

            parse_arg = false;
            continue;
        }

        if (char == '%') {
            parse_arg = true;
            continue;
        }

        writer.writeByte(char) catch {};
    }
}
