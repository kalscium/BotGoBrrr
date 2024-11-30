//! Autonomous routine for the robot

pub mod act;

use logic::{info, odom::OdomState, pid::PIDState};
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
    odom_y: &mut OdomState,
) {
    // move 90cm into the mogo infront
    act::y_coord(900., odom_y, logfile);

    // make sure the robot is straight and then activate the solenoid
    act::rotate(0., logfile);
    act::solenoid(true);

    // activate the belt for around 2 seconds
    act::belt(config::motors::BELT_VOLTS);
    act::wait(2000);
    act::belt(0);

    // turn to -45 degrees and activate belt
    act::rotate(-45., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards about 78cm to grab another ring
    act::y_coord(900. - 780., odom_y, logfile); // grab the ring
    act::rotate(-45., logfile);
    act::wait(2000); // wait for the ring to score
    act::belt(0); // stop the belt

    // rotate to face backwards
    act::rotate(179., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards about 54cm to try grab another ring
    act::y_coord(900. - 780. - 540., odom_y, logfile); // grab the ring
    act::rotate(179., logfile);
    act::wait(2000); // wait for the ring to score
    act::belt(0); // stop the belt

    // rotate to -45 degrees and then move 66cm to hit the pylons
    act::rotate(-45., logfile);
    act::y_coord(720. - 520. - 480. + 660., odom_y, logfile);
    act::rotate(-45., logfile);

    // flush logs
    log::logic_flush(logfile);
}

/// The autonomous routine for autonomous skills runs
fn skills_auton(
    logfile: &mut LogFile,
    odom_y: &mut OdomState,
) {
    act::wait(5000); // wait 5 seconds to calibrate imu

    // move 30cm into the mogo infront
    act::y_coord(300., odom_y, logfile);

    // make sure the robot is straight and then activate the solenoid
    act::rotate(0., logfile);
    act::solenoid(true);

    // activate the belt for around 2 seconds to score
    act::belt(config::motors::BELT_VOLTS);
    act::wait(2000);
    act::belt(0);

    // turn 180 degrees and activate belt
    act::rotate(179., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards about 54cm to grab and score a ring
    act::y_coord(300. - 540., odom_y, logfile);
    act::rotate(179., logfile); // make sure it's still right
    act::wait(2000); // wait for the ring to score
    act::belt(0); // stop the belt

    // move forwards about 54cm to get back to where you were before
    act::y_coord(300., odom_y, logfile);

    // turn to -90 degrees and activate belt
    act::rotate(-90., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards about 90 cm to grab another 2 rings
    act::y_coord(300. - 900., odom_y, logfile);
    act::rotate(-90., logfile); // make sure it's still right
    act::wait(2000); // wait for the ring to score
    act::belt(0); // stop the belt

    // move back forwards another 30 cm and turn to zero degrees before turning on the belt
    act::y_coord(300. - 900. + 300., odom_y, logfile);
    act::rotate(0., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards 26cm to grab yet another ring and score it
    act::y_coord(300. - 900. + 300. - 260., odom_y, logfile);
    act::wait(2000); // wait for the ring to score
    act::belt(0); // stop the belt

    // turn to -75 degrees and go backwards 36cm to push the mogo into the corner and let go
    act::rotate(-75., logfile);
    act::y_coord(300. - 900. + 300. - 260. - 360., odom_y, logfile);
    act::rotate(-75., logfile);
    act::solenoid(false);

    // turn to -18 degrees and go forwards 336cm to grab the next blue mogo
    act::rotate(-18., logfile);
    act::y_coord(-920. + 3360., odom_y, logfile);
    act::rotate(-18., logfile);
    act::solenoid(true);

    // turn to 80 degrees and then go forwards for 110cm to push the mogo into the corner
    act::rotate(80., logfile);
    act::y_coord(-920. + 3360. + 1100., odom_y, logfile);
    act::rotate(80., logfile);
    act::solenoid(false);

    // turn to -95 degrees and then go forwards for 243cm to grab another blue mogo
    act::rotate(-95., logfile);
    act::y_coord(-920. + 3360. + 1100. + 2430., odom_y, logfile);
    act::rotate(-95., logfile);
    act::solenoid(true);

    // turn to -80 degrees and then go forwards for 100cm to push it into the corner
    act::rotate(-80., logfile);
    act::y_coord(-920. + 3360. + 1100. + 2430. + 1000., odom_y, logfile);
    act::rotate(-80., logfile);
    act::solenoid(false);

    // turn to 108 degrees and then go forwards 170cm to grab an empty mogo
    act::rotate(108., logfile);
    act::y_coord(6970. + 1700., odom_y, logfile);
    act::rotate(108., logfile);
    act::solenoid(true);

    // turn to 50 degrees and activate belt
    act::rotate(50., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards 90cm to collect and score a ring
    act::y_coord(5980. + 1700. - 900., odom_y, logfile);
    act::rotate(50., logfile);
    act::wait(2000);
    act::belt(0); // stop belt

    // turn to 90 degrees and then activate belt
    act::rotate(90., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards by 63cm to collect and score a ring
    act::y_coord(5980. + 1700. - 900. - 630., odom_y, logfile);
    act::rotate(90., logfile);
    act::wait(2000);
    act::belt(0); // stop belt

    // turn to 30 degrees and then activate belt
    act::rotate(30., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards by 68cm to collect and score a ring
    act::y_coord(5980. + 1700. - 900. - 630. - 680., odom_y, logfile);
    act::rotate(30., logfile);
    act::wait(2000);
    act::belt(0); // stop belt

    // turn to -25 degrees and then activate belt
    act::rotate(-25., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards by 69cm to collect and score a ring
    act::y_coord(5980. + 1700. - 900. - 630. - 680. - 690., odom_y, logfile);
    act::rotate(-25., logfile);
    act::wait(2000);
    act::belt(0); // stop belt

    // turn to -160 degrees and then go forwards for 101cm to push it into the corner
    act::rotate(-160., logfile);
    act::y_coord(4780. + 1010., odom_y, logfile);
    act::rotate(-160., logfile);
    act::solenoid(false);

    // turn to 68 degrees and then go forwards 112cm to grab an empty mogo
    act::rotate(68., logfile);
    act::y_coord(4780. + 1010. + 1120., odom_y, logfile);
    act::rotate(68., logfile);
    act::solenoid(true);

    // turn to 180 degrees and then activate belt
    act::rotate(179., logfile);
    act::belt(config::motors::BELT_VOLTS);

    // move backwards by 58cm to collect and score a ring
    act::y_coord(4780. + 1010. + 1120. - 580., odom_y, logfile);
    act::rotate(179., logfile);
    act::wait(2000);
    act::belt(0); // stop belt
       
    // flush logs
    log::logic_flush(logfile);
}
