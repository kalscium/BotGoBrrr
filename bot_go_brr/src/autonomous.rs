//! Autonomous routine for the robot

pub mod act;

use logic::{info, intent, odom::OdomState};
use crate::{config, drive, log::{self, LogFile}};

/// The autonomous routine entrypoint
pub fn autonomous() {
    info!("autonomous period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_AUTO_PATH); // filestream to the opcontrol logfile

    // variables for odom
    let mut odom_y = OdomState {
        prev_ly: drive::get_rotation_angle(config::auton::ODOM_LY_PORT),
        prev_ry: drive::get_rotation_angle(config::auton::ODOM_RY_PORT),
        y_coord: 0.,
    };

    #[cfg(not(feature = "skills"))]
    match_auton(&mut logfile, &mut odom_y);
    #[cfg(feature = "skills")]
    skills_auton(&mut logfile, &mut odom_y);
}

/// The autonomous routine for the begining of matches
fn match_auton(
    logfile: &mut LogFile,
    odom: &mut OdomState,
) {
    // to let the program calculate where the robot should relatively be in the y_coord instead of me
    let mut desired_y = 0.;

    // setup
    intent!("open mogo grabber and also drop the intake");
    act::solenoid(false); // disengage the solenoid
    act::belt(-4096); // drop the intake
    act::wait(800, odom);
    act::belt(0);

    intent!("move 120cm into the mogo infront");
    desired_y += 1200.;
    act::goto(0., desired_y, odom, logfile);

    // slowly drive into the mogo for 0.72 seconds before clamping
    act::drive(2048, 2048);
    act::wait(720, odom);
    act::solenoid(true);
    act::drive(0, 0);

    // wait for the solenoid to clamp and then correct for any errors while grabbing the mogo
    act::wait(180, odom); // wait for the thing to fully clamp down
    act::goto(0., desired_y, odom, logfile);

    intent!("activate the belt and wait for it to score the preload before moving on");
    act::belt(config::motors::BELT_VOLTS);
    act::wait(2000, odom); // note that the belt doesn't stop

    intent!("move 60cm backwards at an angle of -90 degrees to grab another ring and wait for it to score");
    desired_y -= 600.;
    act::goto(-90., desired_y, odom, logfile);
    act::wait(2000, odom);
    act::belt(0); // stop belt

    intent!("move 90cm forwards at an angle of -90 degrees to hit the pylons");
    desired_y += 900.;
    act::goto(-90., desired_y, odom, logfile);

    // flush logs
    log::logic_flush(logfile);
}

/// The autonomous routine for autonomous skills runs
fn skills_auton(
    logfile: &mut LogFile,
    odom: &mut OdomState,
) {
    // to let the program calculate where the robot should relatively be in the y_coord instead of me
    let mut desired_y = 0.;

    act::wait(5000, odom); // wait 5 seconds to calibrate imu

    intent!("move 30cm into the mogo infront and then activate the solenoid");
    desired_y += 300.;
    act::goto(0., desired_y, odom, logfile);

    // slowly drive into the mogo for 0.64 seconds before clamping
    act::drive(2048, 2048);
    act::wait(800, odom);
    act::solenoid(true);
    act::drive(0, 0);

    // wait for the solenoid to clamp and then go back with odom
    act::wait(180, odom); // wait for the thing to fully clamp down
    act::goto(0., desired_y, odom, logfile);

    intent!("activate the belt for around 2 seconds to score before moving on");
    act::belt(config::motors::BELT_VOLTS);
    act::wait(2000, odom); // note how the belt is still spinning

    intent!("go backwards about 54cm while facing backwards to grab and wait for another ring to score");
    desired_y -= 540.;
    act::goto(179., desired_y, odom, logfile);
    act::wait(2000, odom); // wait for the ring to score

    intent!("move forwards about 54cm to get back to where you were before");
    desired_y += 540.;
    act::goto(179., desired_y, odom, logfile);

    intent!("move backwards about 90cm at an angle of -90 degrees to grab another 2 rings and wait for them to score");
    desired_y -= 900.;
    act::goto(-90., desired_y, odom, logfile);
    act::wait(2000, odom);

    intent!("move forwards another 30cm to prepare for the next action");
    desired_y += 300.;
    act::goto(-90., desired_y, odom, logfile);

    intent!("move backwards 27cm at an angle of 0 degrees to grab yet another ring and wait for it to score before finally stopping the belt");
    desired_y -= 270.;
    act::goto(0., desired_y, odom, logfile);
    act::wait(2000, odom);
    act::belt(0);

    intent!("move forwards 36cm at an angle of 105 degrees to push the mogo into the corner and let go");
    desired_y += 360.;
    act::goto(105., desired_y, odom, logfile);
    act::solenoid(false);

    intent!("turn to -18 degrees and go forwards 336cm to grab the next blue mogo");
    desired_y += 3360.;
    act::goto(-18., desired_y, odom, logfile);

    // slowly drive into the mogo for 0.64 seconds before clamping
    act::drive(2048, 2048);
    act::wait(800, odom);
    act::solenoid(true);
    act::drive(0, 0);

    // wait for the solenoid to clamp and then go back with odom
    act::wait(180, odom); // wait for the thing to fully clamp down
    act::goto(0., desired_y, odom, logfile);

    intent!("turn to 80 degrees and then go forwards for 110cm to push the mogo into the corner");
    desired_y += 1100.;
    act::goto(80., desired_y, odom, logfile);
    act::solenoid(false);

    intent!("turn to -95 degrees and then go forwards for 243cm to grab another blue mogo");
    desired_y += 2430.;
    act::goto(-95., desired_y, odom, logfile);

    // slowly drive into the mogo for 0.64 seconds before clamping
    act::drive(2048, 2048);
    act::wait(800, odom);
    act::solenoid(true);
    act::drive(0, 0);

    // wait for the solenoid to clamp and then go back with odom
    act::wait(180, odom); // wait for the thing to fully clamp down
    act::goto(0., desired_y, odom, logfile);

    intent!("turn to -80 degrees and then go forwards for 80cm to push it into the corner");
    desired_y += 800.;
    act::goto(-80., desired_y, odom, logfile);
    act::solenoid(false);

    intent!("turn to 108 degrees and then go forwards 180cm to grab an empty mogo");
    desired_y += 1800.;
    act::goto(108., desired_y, odom, logfile);

    // slowly drive into the mogo for 0.64 seconds before clamping
    act::drive(2048, 2048);
    act::wait(800, odom);
    act::solenoid(true);
    act::drive(0, 0);

    // wait for the solenoid to clamp and then go back with odom
    act::wait(180, odom); // wait for the thing to fully clamp down
    act::goto(0., desired_y, odom, logfile);

    intent!("move backwards 90cm at an angle of 50 degrees to grab a ring and wait for it to score");
    desired_y -= 900.;
    act::goto(50., desired_y, odom, logfile);
    act::belt(config::motors::BELT_VOLTS);
    act::wait(2000, odom);

    intent!("move backwards 63cm at an angle of 90 degrees, grab a ring and wait for it to score");
    desired_y -= 630.;
    act::goto(90., desired_y, odom, logfile);
    act::wait(2000, odom);

    intent!("move backwards 68cm at an angle of 30 degrees, grab a right and wait for it to score");
    desired_y -= 680.;
    act::goto(30., desired_y, odom, logfile);
    act::wait(2000, odom);

    intent!("move backwards 69cm at an angle of -25 degrees to collect a ring and wait for it to score before stopping the belt");
    desired_y -= 690.;
    act::goto(-25., desired_y, odom, logfile);
    act::wait(2000, odom);
    act::belt(0); // stop belt

    intent!("turn to -160 degrees and then go forwards for 101cm to push it into the corner");
    desired_y += 1010.;
    act::goto(-160., desired_y, odom, logfile);
    act::solenoid(false);

    intent!("turn to 68 degrees and then go forwards 112cm to grab an empty mogo");
    desired_y += 1120.;
    act::goto(68., desired_y, odom, logfile);

    // slowly drive into the mogo for 0.64 seconds before clamping
    act::drive(2048, 2048);
    act::wait(800, odom);
    act::solenoid(true);
    act::drive(0, 0);

    // wait for the solenoid to clamp and then go back with odom
    act::wait(180, odom); // wait for the thing to fully clamp down
    act::goto(0., desired_y, odom, logfile);
    
    intent!("turn to 180 degrees and then activate belt");
    act::correct_yaw(179., logfile);
    act::belt(config::motors::BELT_VOLTS);

    intent!("move backwards by 58cm to collect a ring and wait for it to score");
    desired_y -= 580.;
    act::goto(179., desired_y, odom, logfile);
    act::wait(2000, odom);
    act::belt(0); // stop belt
       
    // flush logs
    log::logic_flush(logfile);
}
