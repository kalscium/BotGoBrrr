const std = @import("std");

pub fn build(b: *std.Build) void {
    // get the standard config options
    const local_target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // define the vex v5 brain target
    const vex_v5_target = b.resolveTargetQuery(.{
        .cpu_arch = .thumb,
        .os_tag = .freestanding,
        .abi = .eabihf,
        .cpu_model = .{ .explicit = &std.Target.arm.cpu.cortex_a9 },
        .cpu_features_add = std.Target.arm.featureSet(&.{
            .fp16,
            .neonfp,
        }),
    });

    // define the robot code
    const robot_mod = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .target = vex_v5_target,
        .optimize = optimize,
        .strip = true,
        .link_libc = false,
        .unwind_tables = .sync,
    });

    // options
    const asm_opcontrol = b.option(bool, "asm-opcontrol", "Sets whether to use the arm asm version of opcontrol");
    const log_velocity = b.option(bool, "log-velocity", "Sets whether to log the velocity of the robot every tick");
    const log_bench = b.option(bool, "benchmark", "Sets whether to log cycle times for benchmarking");
    const tune = b.option([]const u8, "tune", "Sets the kind of tuning (instead of opcontrol) you wish to do");
    const debug_mode = b.option(bool, "dbgmode", "Sets whether to enable 'debug mode' opcontrol, for debugging, tuning, planning, and showcasing auton (pure pursuit)");
    const debug_mode_pid = b.option(bool, "dbgmode-pid", "Sets whether to enable 'debug mode' for PIDs");
    const auton_routine = b.option([]const u8, "auton-routine", "Sets the auton routine to use/compile.");
    const w_akibot = b.option(bool, "w-akibot", "Use if allianced with akibot (wait 2s auton)");

    // options set
    var options = b.addOptions();
    options.addOption(bool, "asm_opcontrol", asm_opcontrol orelse false);
    options.addOption(bool, "log_velocity", log_velocity orelse false);
    options.addOption(bool, "benchmark", log_bench orelse false);
    options.addOption(?[]const u8, "tune", tune);
    options.addOption(bool, "debug_mode", debug_mode orelse false);
    options.addOption(bool, "debug_mode_pid", debug_mode_pid orelse false);
    options.addOption(bool, "w_akibot", w_akibot orelse false);
    options.addOption(?[]const u8, "auton_routine", auton_routine);
    robot_mod.addOptions("options", options);

    // define the pros module
    const pros_mod = b.createModule(.{
        .root_source_file = b.path("include/pros.zig"),
    });

    // add the pros header files and module
    pros_mod.addIncludePath(b.path("include"));
    robot_mod.addImport("pros", pros_mod);

    // create the object
    const robot_obj = b.addObject(.{
        .name = "BotGoBrrr",
        .root_module = robot_mod,
    });

    // install the object (for the makefile)
    const obj_install = b.addInstallBinFile(robot_obj.getEmittedBin(), "userlib.zig.o");
    const install_obj = b.step("obj", "Builds the userlib as a .o file");
    install_obj.dependOn(&obj_install.step);
    b.getInstallStep().dependOn(&obj_install.step);

    // create the test binary
    const test_mod = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .target = local_target,
    });
    const test_exe = b.addTest(.{
        .root_module = test_mod,
    });
    test_exe.root_module.addImport("pros", pros_mod);
    test_exe.root_module.addOptions("options", options);

    // add the stub library for the test and link it
    const stubs_mod = b.createModule(.{
        .root_source_file = b.path("include/stub.zig"),
        .target = local_target,
        .optimize = optimize,
    });
    const stubs_lib = b.addLibrary(.{
        .linkage = .static,
        .name = "stubs",
        .root_module = stubs_mod,
    });
    test_exe.linkLibrary(stubs_lib);

    // add a test step for the userlib
    const test_step = b.step("test", "Run unit tests");
    const run_unit_tests = b.addRunArtifact(test_exe);
    test_step.dependOn(&run_unit_tests.step);

    // create the simulation binary
    const sim_mod = b.createModule(.{
        .root_source_file = b.path("src/simulation.zig"),
        .target = local_target,
        .optimize = optimize,
    });
    const sim_exe = b.addExecutable(.{
        .name = "simulation",
        .root_module = sim_mod,
    });
    const sim_robot_mod = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .target = local_target,
        .optimize = optimize,
    });
    sim_mod.addImport("pros", pros_mod);
    sim_robot_mod.addImport("pros", pros_mod);
    sim_robot_mod.addOptions("options", options);
    const sim_robot_obj = b.addObject(.{
        .name = "sim_robot_code",
        .root_module = sim_robot_mod,
    });

    // simulation link step
    sim_exe.addObject(sim_robot_obj);

    // add a simulation step
    const sim_step = b.step("simulate", "runs the robot code in a simulated Vex VRC brain");
    const run_sim = b.addRunArtifact(sim_exe);
    sim_step.dependOn(&run_sim.step);
}
