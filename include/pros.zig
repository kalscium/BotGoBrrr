//! Pros api bindings

const std = @import("std");

// PROS headers
pub const adi = @cImport(@cInclude("pros/adi.h"));
pub const apix = @cImport(@cInclude("pros/apix.h"));
pub const colors = @cImport(@cInclude("pros/colors.h"));
pub const device = @cImport(@cInclude("pros/device.h"));
pub const distance = @cImport(@cInclude("pros/distance.h"));
pub const pros_error = @cImport(@cInclude("pros/error.h"));
pub const ext_adi = @cImport(@cInclude("pros/ext_adi.h"));
pub const gps = @cImport(@cInclude("pros/gps.h"));
pub const imu = @cImport(@cInclude("pros/imu.h"));
pub const link = @cImport(@cInclude("pros/link.h"));
pub const misc = @cImport(@cInclude("pros/misc.h"));
pub const motors = @cImport(@cInclude("pros/motors.h"));
pub const optical = @cImport(@cInclude("pros/optical.h"));
pub const rotation = @cImport(@cInclude("pros/rotation.h"));
pub const rtos = @cImport(@cInclude("pros/rtos.h"));
pub const screen = @cImport(@cInclude("pros/screen.h"));
pub const serial = @cImport(@cInclude("pros/serial.h"));
pub const vision = @cImport(@cInclude("pros/vision.h"));

// liblvgl
pub const liblvgl = @cImport(@cInclude("liblvgl/lvgl.h"));

// simple IO ffi bindings
pub extern fn fopen(filename: [*:0]const u8, mode: [*:0]const u8) callconv(.C) ?*std.c.FILE;
pub extern fn fclose(stream: *std.c.FILE) callconv(.C) c_int;
pub extern fn feof(stream: *std.c.FILE) callconv(.C) c_int;
pub extern fn ferror(stream: *std.c.FILE) callconv(.C) c_int;
pub extern fn fread(ptr: [*]const u8, size: isize, nmemb: isize, stream: *std.c.FILE) callconv(.C) c_int;
pub extern fn freopen(filename: [*:0]const u8, mode: [*:0]const u8, stream: *std.c.FILE) callconv(.C) ?*std.c.FILE;
pub extern fn fseek(stream: *std.c.FILE, offset: c_long, whence: c_int) callconv(.C) c_int;
pub extern fn ftell(stream: *std.c.FILE) callconv(.C) c_long;
pub extern fn fwrite(ptr: [*]const u8, stream: *std.c.FILE) callconv(.C) c_long;
pub extern fn remove(stream: *std.c.FILE) callconv(.C) c_int;
pub extern fn rewind(stream: *std.c.FILE) callconv(.C) void;
pub extern fn printf(fmt: [*:0]const u8, ...) callconv(.C) c_int;
pub extern fn fprintf(stream: *std.c.FILE, fmt: [*:0]const u8, ...) callconv(.C) c_int;
pub extern fn scanf(fmt: [*:0]const u8, ...) callconv(.C) c_int;
pub extern fn fscanf(stream: *std.c.FILE, fmt: [*:0]const u8, ...) callconv(.C) c_int;
pub extern fn fgetc(stream: *std.c.FILE) callconv(.C) c_int;
pub extern fn fgets(str: [*:0]u8, n: c_int, stream: *std.c.FILE) callconv(.C) [*:0]u8;
pub extern fn fputc(char: c_int, stream: *std.c.FILE) callconv(.C) c_int;
pub extern fn fputs(str: [*:0]const u8, stream: *std.c.FILE) callconv(.C) c_int;
pub extern fn getchar() callconv(.C) c_int;
pub extern fn putchar(char: c_int) callconv(.C) c_int;
pub extern fn puts(str: [*:0]const u8) callconv(.C) c_int;

/// Returns a mutable pointer to the C/C++ `errno` value
pub extern fn __errno() callconv(.C) *i32;

/// Returns the stdout file stream that's owned by the caller
pub inline fn stdout() *?std.c.FILE { return fopen("sout", "w"); }
/// Returns the stderr file stream that's owned by the caller
pub inline fn stderr() *?std.c.FILE { return fopen("serr", "w"); }
/// Returns the stdin file stream that's owned by the caller
pub inline fn stdin() *?std.c.FILE { return fopen("stin", "r"); }
