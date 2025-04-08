//! Sketchy handwritten vector maths & linear algebra

const std = @import("std");
const odom = @import("odom.zig");

/// Finds the magnitude of a vector
pub inline fn calMag(comptime T: type, vec: @Vector(2, T)) T {
    return std.math.sqrt(std.math.pow(T, vec[0], 2) + std.math.pow(T, vec[1], 2));
}

/// Condenses a vector condition into a single boolean
pub fn vecCond(cond: @Vector(2, bool)) bool {
    return cond[0] and cond[1];
}

/// Convert a direction (in radians) and magnitude to a xy vector
pub fn polarToCartesian(mag: f64, dir: f64) odom.Coord {
    return odom.Coord{
        @sin(dir) * mag,
        @cos(dir) * mag,
    };
}

test polarToCartesian {
    // test some conversions
    std.debug.assert(vecCond(odom.Coord{0, 12} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(0)))));
    std.debug.assert(vecCond(odom.Coord{8, 8} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(45)))));
    std.debug.assert(vecCond(odom.Coord{12, 0} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(90)))));
    std.debug.assert(vecCond(odom.Coord{-8, 8} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(-45)))));
    std.debug.assert(vecCond(odom.Coord{-12, 0} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(-90)))));
    std.debug.assert(vecCond(odom.Coord{-8, -8} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(-135)))));
    std.debug.assert(vecCond(odom.Coord{0, -12} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(-180)))));
}
