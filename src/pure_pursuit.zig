//! Functions for performing my own implementation of 'pure pursuit' (my own algorithm)

const std = @import("std");
const odom = @import("odom.zig");
const vector = @import("vector.zig");
const pros = @import("pros");
const port = @import("port.zig");
const auton = @import("autonomous.zig");
const drive = @import("drive.zig");

/// The width of the robot in mm
pub const robot_width: comptime_float = 290.0; // placeholder

/// The smallest possible f64 value that's just outside of precision.
/// So, this will be automatically removed/lost if added to a value
/// that isn't 0 (evil bithack)
const smallest_f64: f64 = 1e-20;

/// A convenience state machine for following a path until the end of a path is
/// reached and within a precision threshold (auton).
/// 
/// Uses the auton cycle speed and other sensor info, auton pure_pursuit
/// parameters, and odom state to do calculations.
/// 
/// Also, runs 'in_reverse' in that it follows the path with the back of the robot.
pub fn autonFollowPath(path: []const odom.Coord, in_reverse: bool, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // state machine state
    var now = pros.rtos.millis();
    var last_end: usize = 0;
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // we make a copy of the odom state so the changes made by driving in
        // reverse don't mess up the actual odom calculations.
        // 
        // odom is calculated normally, and then acted upon as if it were in
        // reverse
        var odom_copy = odom_state.*;
        if (in_reverse) {
            // we need to make the robot think the back is the front
            // the actual turning velocities and coordinates are unaffected
            // just the yaw itself must change by subtracting 180 degrees
            // also, the vertical and lateral velocities and values must be
            // inverted
            odom_copy.prev_yaw = odom.minimalAngleDiff(std.math.pi, odom_copy.prev_yaw);
            odom_copy.mov_ver_vel = -odom_copy.mov_ver_vel;
            odom_copy.mov_lat_vel = -odom_copy.mov_lat_vel;
        }

        // if it's within precision, break
        if (vector.calMag(f64, path[last_end] - odom_copy.coord) < auton.precision_mm)
            break;

        const params = auton.pure_pursuit_params;

        // calculate the robot's predicted location and base all future calculations off of it
        const predicted = predictCoordYaw(odom_copy, params.lookahead_window);

        // pick the next path points
        const path_seg_start = pickPathPoints(predicted.coord, params.search_radius, path, &last_end);

        // interpolate the goal point
        const goal_point = interpolateGoal(predicted.coord, params.search_radius, path_seg_start, path[last_end]);

        // calculate the left and right drive ratios
        const ratios = followArc(predicted.coord, goal_point, predicted.yaw);

        // calculate the speed for the robot
        const speed = speedController(predicted.coord, predicted.yaw, goal_point, params);

        // find the left and right drive velocities from combining the speed and ratios
        // and then drive the robot at those values
        var ldr, var rdr = ratios * @as(@Vector(2, f64), @splat(speed));

        // if driving in reverse, then switch the left and right and invert them
        if (in_reverse) {
            const ldr_tmp = ldr;
            ldr = -rdr;
            rdr = -ldr_tmp;
        }

        // drive the changes made
        drive.driveLeft(rdr, port_buffer);
        drive.driveRight(ldr, port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.tick_delay);
    }
}

/// Picks the end point based on the last end point (index into the
/// path array), search radius (in mm), robot coordinate and path
/// points array and write it to `last_end`.
///
/// Then picks the start point, as the point immedietely before the end
/// point in the path points array (otherwise robot coord if end is
/// start) and returns it.
pub fn pickPathPoints(coord: odom.Coord, search_radius: f64, path: []const odom.Coord, last_end: *usize) odom.Coord {
    for (path[last_end.*..], last_end.*..) |point, idx| {
        // check if the path point is outside the search radius
        // (if so, then that's your end point, with the start point the index immedietly before)
        if (vector.calMag(f64, point - coord) > search_radius) {
            last_end.* = idx;
            break;
        }
    } else {
        // if the end of the path is reached and no suitable end is found, just used the end of the path
        last_end.* = path.len-1;
    }

    // if the start is the end, just return the robot's current position, otherwise, the point just before the end
    if (last_end.* == 0)
        return coord
    else
        return path[last_end.*-1];
}

test pickPathPoints {
    var coord = odom.Coord{ 1, 1 };
    const search_radius = 2;
    const path: []const odom.Coord = &.{ .{ 1, 2 }, .{1, 1}, .{2, 2}, .{4, 2}, .{5, 1}, .{5, 8} };
    var last_end: usize = 0;
    std.debug.assert(@reduce(.And, pickPathPoints(coord, search_radius, path, &last_end) == odom.Coord{ 2, 2 }));
    std.debug.assert(last_end == 3);
    coord = path[last_end];
    std.debug.assert(@reduce(.And, pickPathPoints(coord, search_radius, path, &last_end) == odom.Coord{ 5, 1 }));
    std.debug.assert(last_end == 5);
}

/// Interpolates a goal coordinate from the path start point and end
/// points, the robot coordinate and also the search radius (for
/// calculating an extension)
pub fn interpolateGoal(coord: odom.Coord, search_radius: f64, start_point: odom.Coord, end_point: odom.Coord) odom.Coord {
    // make all the coordinates relative to the path start
    const rel_coord = coord - start_point;
    const rel_end = end_point - start_point;
    const rel_end_mag = vector.calMag(f64, rel_end);

    // cast the relative robot coordinate onto the direction of the relative (to path start) path end
    const normalized_rel_end = rel_end / @as(odom.Coord, @splat(rel_end_mag));
    const casted_mag = vector.dotProduct(f64, rel_coord, normalized_rel_end);
    const casted_lerp_t = std.math.clamp(casted_mag / rel_end_mag, 0, 1);
    const casted_rel_coord = rel_end * @as(odom.Coord, @splat(casted_lerp_t));

    // calcuate the extension from a^2 = c^2 - b^2
    // where c is the search radius, b is the distance between the real
    // robot coord and the casted robot coord, and a is the extension's
    // magnitude
    const real_casted_distance = vector.calMag(f64, casted_rel_coord - rel_coord);
    const extension_mag = @sqrt(@max(0, search_radius * search_radius - real_casted_distance * real_casted_distance));

    // calculate the final real goal coordinate from the casted
    // magnitude and extension magnitude
    const goal_lerp_t = std.math.clamp((casted_mag + extension_mag) / rel_end_mag, 0, 1);
    const goal_point = start_point + rel_end * @as(odom.Coord, @splat(goal_lerp_t));

    return goal_point;
}

test interpolateGoal {
    const coord = odom.Coord{ 2, 5 };
    const search_radius = 2;
    const start_point = odom.Coord{ 0, 1 };
    const end_point = odom.Coord{ 10, 10 };
    const goal = interpolateGoal(coord, search_radius, start_point, end_point);
    std.debug.assert(@reduce(.And, @round(goal) == odom.Coord{ 4.0, 5.0 }));
}


/// Sets the drivetrain ratio to drive in an arc that connects the
/// current coordinate and goal coordinate, whilst keeping a constant
/// angular velocity (robot yaw as tangent line).
pub fn followArc(coord: odom.Coord, goal: odom.Coord, yaw: f64) @Vector(2, f64) {
    // get the goal vector relative to the robot's current coordinate
    const rel_goal = goal - coord;

    // find the angle of the goal relative to the current coord
    const path_seg_angle = vector.calDir(f64, rel_goal);

    // find theta, the angular distance of the yaw and angle of the path segment
    const theta = odom.minimalAngleDiff(yaw, path_seg_angle);
    const theta_sign = std.math.sign(theta);

    // if theta is larger than 90 degrees (in radians that's pi/2), then just
    // rotate towards the path segment angle
    if (theta > comptime std.math.pi/2.0)
        return .{ 1, -1 }
    else if (theta < comptime -std.math.pi/2.0)
        return .{ -1, 1 };

    // calculate the length of the path segment
    const path_seg_len = vector.calMag(f64, rel_goal);

    // calculate the radius of the arc
    const arc_radius = path_seg_len / (2.0 * @sin(theta) * theta_sign + smallest_f64); // not quite dividing by zero, but value small enough, that the precision loss with non-zero values will remove it (won't affect anything)

    // calculate the approximate change in distance for the left and right drives
    // then calculate the ratio between the two, that's the ratio we'll use
    // left offset = +base/2
    // right offset = -base/2
    const delta_left = arc_radius + robot_width/2.0 * theta_sign;
    const delta_right = arc_radius - robot_width/2.0 * theta_sign;
    // const delta_total = delta_left + delta_right;
    const delta_max = @max(@abs(delta_left), @abs(delta_right));

    // calculate the raw left and right drives
    const ldr = delta_left / delta_max;
    const rdr = delta_right / delta_max;

    return .{ ldr, rdr };
}

test followArc {
    const ldr, const rdr = followArc(.{ -2, 1 }, .{ -2, 10 }, std.math.degreesToRadians(0));
    std.debug.assert(ldr == 1 and rdr == 1);
}

/// Calculates the robot's predicted location & yaw based upon the lookahead
/// window and also odom state
pub fn predictCoordYaw(odom_state: odom.State, lookahead_window: f64) struct{ coord: odom.Coord, yaw: f64 } {
    // calculate the predicted robot location (from velocity & SUVATs)
    const predicted_yaw = odom_state.prev_yaw + odom_state.rot_vel * lookahead_window;
    const predicted_coord =
        odom_state.coord + // current location
        vector.polarToCartesian(odom_state.mov_ver_vel * lookahead_window, predicted_yaw) + // predicted vertical movement
        vector.polarToCartesian(odom_state.mov_lat_vel * lookahead_window, predicted_yaw + comptime std.math.degreesToRadians(90)) // predicted lateral movement
    ;

    return .{ .coord = predicted_coord, .yaw = predicted_yaw };
}

test predictCoordYaw {
    // example odom state
    var state: odom.State = undefined;
    state.mov_ver_vel = 1;
    state.mov_lat_vel = 0.5;
    state.coord = .{ 10, 10 };
    state.rot_vel = std.math.degreesToRadians(10);

    // predict the new coord
    const new = predictCoordYaw(state, 3); // 3 ms into the future
    // std.debug.print("new coord: {d}\nnew_yaw: {d}\n", .{ new.coord, std.math.radiansToDegrees(new.yaw) });
    std.debug.assert(@reduce(.And, @round(new.coord) == @as(odom.Coord, .{ 13, 12 })));
    std.debug.assert(@round(std.math.radiansToDegrees(new.yaw)) == 30);
}

/// Calculates the velocity multiplier (from 0..=1) for pure pursuit
/// from the robot's coord & yaw, goal positions, and tuned parameters
pub fn speedController(coord: odom.Coord, yaw: f64, goal: odom.Coord, params: Parameters) f64 {
    // calculate the distance & yaw errors between the coord and goal
    const rel_goal = goal - coord;
    const distance_err = vector.calMag(f64, rel_goal); // distance will always be less or equal to the search radius
    const rel_goal_angle = vector.calDir(f64, rel_goal);
    const yaw_err = odom.minimalAngleDiff(yaw, rel_goal_angle);

    // calculate the proportional term of the controller
    const prop = distance_err / params.search_radius * params.kp;

    // calculate the turning speed 'multiplier' (actually a lerp so that the
    // robot turns at the specified 180 degree turning speed)
    const turn_t = yaw_err / std.math.pi;
    const turning_mul = std.math.lerp(1.0, params.turn_speed_180, turn_t);

    // calculate the final turning speed by multiplying together the multipliers
    return prop * turning_mul;
}

/// The tune-able parameters for pure pursuit, all in one place for convenience
pub const Parameters = struct {
    /// The most blatant and obvious thing you need to tune in the pure pursuit
    /// controller, is the search radius
    /// (aka the lookahead distance (in mm) for the path).
    ///
    /// Start off with the distance between the middle and front of the robot,
    /// then keep either increasing or decreasing so that it does a 90 degree
    /// turn (ignoring overshoot & speed) with a tight-enough or loose-enough
    /// turn circle (good balance between accuracy and smoothness).
    search_radius: f64,

    /// The proportional distance error speed multiplier for the robot (0..=1).
    /// 
    /// Or in other words, the speed the robot travels at normally, unless
    /// stopping or turning.
    ///
    /// Tune by getting the robot to drive in a straight line and decrease it
    /// from 1 until it's a reasonable speed.
    kp: f64,

    /// The prediction window (in ms) for the robot
    /// (should be a multiplier of the cycle time)
    /// 
    /// How far into the future the robot will predict and act upon.
    /// 
    /// Let it be the last thing you tune, and just keep increasing from zero
    /// until most of the overshoot due to inertia is gone, but not too much as
    /// to make the robot jitter
    /// (where it fights itself and goes forwards and backwards really quickly)
    lookahead_window: f64,

    /// The minimum turning speed multiplier (from 0..=1)
    /// 
    /// The speed multiplier that is applied (through a lerp) to the robot when
    /// it's doing a 180 degree turn.
    /// 
    /// Tune by having the robot make a 180 degree turn, and decrease from 1
    /// until the turn doesn't drift too much (due turning circle too large, NOT inertia).
    turn_speed_180: f64,
};

test "robot forwards kinematics simulation" {
    // so far, no physics in this simulation, which also means no PID & curvature
    // speed controller either (no physically accurate stopping lol)

    // simulation configs
    const max_cycle = 10000; // simulation force quits after this many iterations (gives up)
    // the parameters for pure pursuit
    const params = Parameters{
        .search_radius = 20,
        .kp = 1.0,
        .turn_speed_180 = 0.4,
        .lookahead_window = 0.0,
    };
    const path: []const odom.Coord = &.{
        .{ 40, 40 },
        .{ 40, 120 },
        .{ 200, 120 },
        .{ 140, 60 },
        .{ 240, 60 },
        .{ 300, 100 },
        .{ 360, 20 },
        .{ 420, 100 },
        .{ 480, 20 },
    };

    // open output file
    var file = try std.fs.cwd().createFile("pp_forward_kinematic_sim.csv", .{});
    defer file.close();
    try file.writer().writeAll("yaw,x,y,goal_x,goal_y,true_path_x,true_path_y,ldr,rdr,speed_mul\n");

    var now: usize = 0;
    var coord: odom.Coord = .{ 0, 0 };
    var yaw: f64 = std.math.degreesToRadians(-20);
    var last_end: usize = 0;
    while (now < max_cycle) : (now += 1) {
        // get the ldr & rdr from pure pursuit
        const path_start = pickPathPoints(coord, params.search_radius, path, &last_end);
        const path_end = path[last_end];
        const goal_coord = interpolateGoal(coord, params.search_radius, path_start, path_end);
        const speed = speedController(coord, yaw, goal_coord, params);
        const ldr, const rdr = followArc(coord, goal_coord, yaw);

        // log the logs
        try file.writer().print("{d},{d},{d},{d},{d},{d},{d},{d},{d},{d}\n", .{
            std.math.radiansToDegrees(yaw),
            coord[0],
            coord[1],
            goal_coord[0],
            goal_coord[1],
            path[@min(path.len-1,now)][0],
            path[@min(path.len-1,now)][1],
            ldr,
            rdr,
            speed,
        });

        // if the goal is reached, break
        if (vector.calMag(f64, path_end - coord) < @import("autonomous.zig").precision_mm)
            break;

        // forward kinematics
        const angular_vel = (ldr - rdr)/robot_width;
        const tangent_vel = (ldr + rdr)/2;
        yaw += angular_vel;
        if (yaw > std.math.pi)
            yaw -= std.math.tau
        else if (yaw < -std.math.pi)
            yaw += std.math.tau;
        coord += .{
            @sin(yaw) * tangent_vel,
            @cos(yaw) * tangent_vel,
        };
    } else {
        // if the robot never reaches it's goal
        std.debug.panic("pure pursuit fails to reach goal after {} cycles\n", .{max_cycle});
    }
}
