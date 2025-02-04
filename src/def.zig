//! A bunch of PROS definitions/macros

const std = @import("std");

/// The controller you read from
pub const Controller = enum(c_uint) {
    master,
    partner,
};

/// An analog joystick on the controller
pub const ControllerAnalog = enum(c_uint) {
    left_x,
    left_y,
    right_x,
    right_y,
};

/// A digital button on the controller
pub const ControllerDigital = enum(c_uint) {
    l1 = 6,
    l2,
    r1,
    r2,
    up,
    down,
    left,
    right,
    x,
    b,
    y,
    a,
};

/// The PROS i32 error code
pub const pros_err_i32: i32 = std.math.maxInt(i32);
/// The PROS f64 error code
pub const pros_err_f64: f64 = std.math.inf(f64);

/// PROS errno error codes
pub const pros_error_code = struct {
    pub const none = 0;
    pub const enoent = 2;
    pub const eintr = 4;
    pub const eio = 5;
    pub const enxio = 6;
    pub const ebadf = 9;
    pub const eagain = 11;
    pub const enomem = 12;
    pub const eacces = 13;
    pub const efault = 14;
    pub const ebusy = 16;
    pub const eexist = 17;
    pub const exdev = 18;
    pub const enodev = 19;
    pub const enotdir = 20;
    pub const eisdir = 21;
    pub const einval = 22;
    pub const enospc = 28;
    pub const espipe = 29;
    pub const erofs = 30;
    pub const eunatch = 42;
    pub const ebade = 50;
    pub const eftype = 79;
    pub const enmfile = 89;
    pub const enotempty = 90;
    pub const enametoolong = 91;
    pub const eopnotsupp = 95;
    pub const enobufs = 105;
    pub const enoprotoopt = 109;
    pub const eaddrinuse = 112;
    pub const etimedout = 116;
    pub const einprogress = 119;
    pub const ealready = 120;
    pub const eaddrnotavail = 125;
    pub const eisconn = 127;
    pub const enotconn = 128;
    pub const enomedium = 135;
    pub const eilseq = 138;
    pub const ecanceled = 140;
};
