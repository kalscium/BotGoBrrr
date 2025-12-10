#include "auton.hpp"
#include "drive.hpp"
#include "api.h"
#include "lemlib/api.hpp"
#include "tower.hpp"

extern lemlib::Chassis chassis;
extern pros::MotorGroup tower_middle_intake;
extern pros::MotorGroup tower_hood_storage;
extern pros::adi::DigitalOut little_will_pnu;

void autonTune() {
        chassis.setPose(0, 0, 0);

        while (true) {
                chassis.turnToHeading(45, 1000);
                pros::delay(500);
                chassis.turnToHeading(180, 1000);
                pros::delay(1600);
                chassis.turnToHeading(0, 1000);
                break;

                chassis.moveToPoint(0, 39, 2000);
                pros::delay(1600);
                chassis.moveToPoint(0, 0, 2000, { .forwards=false });
                chassis.turnToHeading(0, 1000);
        }
}

void autonLeft() {
        // init stuff
        chassis.setPose(0, 0, 0);
        TowerState tower;

        // move into the blocks and intake them
        tower.storeBlocks(1.0);
        chassis.moveToPoint(-18.29, 28.573, 2000); // jerry
        pros::delay(540);
        little_will_pnu.set_value(true);

        // move over to and line up to matchload
        chassis.moveToPoint(-11.374, 16.016, 4000, { .forwards=false });
        chassis.moveToPoint(-39.8375, -1.456, 4000);
        chassis.turnToHeading(-180, 1000);

        // move into the matchloader and intake before moving out
        chassis.moveToPoint(-39.8375, -15.7, 2000, {}, false); // jerry
        pros::delay(1800);
        tower.storeBlocks(0.0);
        chassis.moveToPoint(-39.8375, -1.456, 4000, { .forwards=false }, false);
        little_will_pnu.set_value(false);

        // line up to long-goal and score
        chassis.turnToHeading(0, 1000);
        chassis.moveToPoint(-40.2375, 15.47, 2000, {}, false);
        chassis.turnToHeading(0, 1000);

        // anti-jam and score
        tower.scoreBottom(0.0);
        pros::delay(350);
        tower.scoreBottom(0.0);
        pros::delay(150);
        tower.scoreTop(1.0);
}

void autonRight() {
        // init stuff
        chassis.setPose(0, 0, 0);
        TowerState tower;

        // move into the blocks and intake them
        tower.storeBlocks(1.0);
        // chassis.moveToPoint(18.29, 28.573, 2000);
        chassis.moveToPoint(19.85, 27.932, 2000);
        pros::delay(540);
        little_will_pnu.set_value(true);

        // move over to and line up to matchload
        chassis.moveToPoint(11.374, 16.016, 4000, { .forwards=false });
        chassis.moveToPoint(39.3375, -1.456, 4000);
        chassis.turnToHeading(-180, 2000);

        // move into the matchloader and intake before moving out
        chassis.moveToPoint(40.8375, -16, 4000, {}, false); // jerry
        pros::delay(500);
        tower.storeBlocks(0.0);
        chassis.moveToPoint(40.8375, -1.456, 4000, { .forwards=false }, false);
        little_will_pnu.set_value(false);

        // line up to long-goal and score
        chassis.turnToHeading(0, 1000);
        chassis.moveToPoint(38.7, 10, 2000, {}, false);
        chassis.turnToHeading(0, 1000, {}, false);

        // anti-jam and score
        tower.scoreTop(1.0);
        pros::delay(480);
        tower.scoreBottom(1.0);
        pros::delay(280);
        tower.scoreTop(1.0);
}
