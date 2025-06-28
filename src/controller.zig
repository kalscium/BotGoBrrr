//! Convenience functions for safely reading from controllers

const pros = @import("pros");
const def = @import("def.zig");

/// Reads a digital controller input and returns if it's currently being held down
pub fn get_digital(button: pros.misc.controller_digital_e_t) bool {
    return pros.misc.controller_get_digital(pros.misc.E_CONTROLLER_MASTER, button) == 1;
}

/// Reads a digital controller input and returns if it's been newly pressed (no holding repeats)
pub fn get_digital_new_press(button: pros.misc.controller_digital_e_t) bool {
    return pros.misc.controller_get_digital_new_press(pros.misc.E_CONTROLLER_MASTER, button) == 1;
}

/// Reads an analog controller channel and returns it's value from -127..=127, returning zero upon controller disconnect
pub fn get_analog(channel: pros.misc.controller_analog_e_t) i8 {
    const raw = pros.misc.controller_get_analog(pros.misc.E_CONTROLLER_MASTER, channel);
    if (raw == def.pros_err_i32)
        return 0
    else
        return @intCast(raw);
}
