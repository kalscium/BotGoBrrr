//! Sketchy handwritten vector maths & linear algebra

const std = @import("std");
const odom = @import("odom.zig");

/// Finds the magnitude of a vector
pub inline fn calMag(comptime T: type, vec: @Vector(2, T)) T {
    return @sqrt(std.math.pow(f64, vec[0], 2) + std.math.pow(f64, vec[1], 2));
}

/// Convert a direction (in radians) and magnitude to a xy vector
pub fn polarToCartesian(mag: f64, dir: f64) odom.Coord {
    return odom.Coord{
        @sin(dir) * mag,
        @cos(dir) * mag,
    };
}

/// Finds the dotproduct of two vectors
pub inline fn dotProduct(comptime T: type, a: @Vector(2, T), b: @Vector(2, T)) T {
    return a[0] * b[0] + a[1] * b[1];
}

test polarToCartesian {
    // test some conversions
    std.debug.assert(@reduce(.And, odom.Coord{0, 12} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(0)))));
    std.debug.assert(@reduce(.And, odom.Coord{8, 8} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(45)))));
    std.debug.assert(@reduce(.And, odom.Coord{12, 0} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(90)))));
    std.debug.assert(@reduce(.And, odom.Coord{-8, 8} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(-45)))));
    std.debug.assert(@reduce(.And, odom.Coord{-12, 0} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(-90)))));
    std.debug.assert(@reduce(.And, odom.Coord{-8, -8} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(-135)))));
    std.debug.assert(@reduce(.And, odom.Coord{0, -12} == std.math.round(polarToCartesian(12, comptime std.math.degreesToRadians(-180)))));
}
