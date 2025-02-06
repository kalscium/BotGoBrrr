//! Functions for dealing with ports on the robot brain

const std = @import("std");

/// Returns a port that's comptime checked (1-21)
pub fn checkedPort(comptime port: comptime_int) comptime_int {
    // check if the port is valid
    if (port < 1 or port > 21) {
        @compileError("motor port must be within the range 1..=21");
    }

    return port;
}

/// The port buffer is a bitfield of ports on the robot brain (like the
/// name suggests) and whether they are connected/working or not at
/// this current point in time
/// 
/// For code to work, the layout of this struct must not change
pub const PortBuffer = packed struct(u24) {
    port_1: bool,
    port_2: bool,
    port_3: bool,
    port_4: bool,
    port_5: bool,
    port_6: bool,
    port_7: bool,
    port_8: bool,
    port_9: bool,
    port_10: bool,
    port_11: bool,
    port_12: bool,
    port_13: bool,
    port_14: bool,
    port_15: bool,
    port_16: bool,
    port_17: bool,
    port_18: bool,
    port_19: bool,
    port_20: bool,
    port_21: bool,
    __padding: u3,

    /// Writes a boolean value based upon a port value
    pub fn portWrite(self: *PortBuffer, comptime port: comptime_int, val: bool) void {
        @field(self, std.fmt.comptimePrint("port_{}", .{ @abs(port) })) = val;
    }
};
