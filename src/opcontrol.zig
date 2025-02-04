//! Defines the driver-control routine

const pros = @import("pros");
const def = @import("def.zig");
const drive = @import("drive.zig");

/// Gets called during the driver-control period
export fn opcontrol() callconv(.C) void {
    // open the motor disconnect file
    const mtr_dscnt_file = pros.fopen("/usd/motor_disconnects.bin", "a");
    defer if (mtr_dscnt_file) |file| {
        _ = pros.fclose(file);
    };

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
    var disconnect_buffer: [drive.drive_mtr_side_cnt * 2]i8 = [_]i8{0} ** (drive.drive_mtr_side_cnt * 2);
    drive.driveLeft(ldr, @ptrCast(&disconnect_buffer));
    drive.driveRight(rdr, @ptrCast(@as([*]i8, &disconnect_buffer)+drive.drive_mtr_side_cnt));
    
    // logs
    if (mtr_dscnt_file) |file| _ = pros.fwrite(@ptrCast(&disconnect_buffer), @sizeOf(u8), disconnect_buffer.len, file);
}
