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
        // chassis.moveToPoint(0, 12.0, 2000);
        // chassis.turnToHeading(-45, 1000);
        // tower.storeBlocks(1.0); // loud
        // chassis.moveToPoint(-16, 34, 2000);
        chassis.moveToPoint(-16.5, 25.3, 2000); // jerry
        pros::delay(540);
        little_will_pnu.set_value(true); // loud

        // // line up and score mid goal
        // chassis.moveToPoint(-7.3, 33.27, 2000);
        //         // unjam
        //         tower.scoreBottom(1.0);
        //         pros::delay(290);
        //         tower.storeBlocks(1.0);
        // chassis.turnToHeading(45, 1000, {}, false);
        // tower.scoreTop(0.7);
        // pros::delay(320);
        // tower.storeBlocks(1);

        // // new middle score
        // chassis.turnToHeading(45, 1000, {}, false);
        // if (true) return;
        // chassis.moveToPoint(-11.7, 28.7, 2000);
        // tower.scoreTop(0.7);
        // // pros::delay(320);
        // pros::delay(1000);
        // tower.storeBlocks(1);

        // move over to and line up to matchload
        // chassis.moveToPoint(-40, 0.5, 2000);
        chassis.moveToPoint(-39, -5, 4000, { .forwards = false }); // jerry
        chassis.turnToHeading(-170, 1000);

        // move into the matchloader and intake before moving out
        chassis.moveToPoint(-41.7, -14.7, 2000, {}, false); // jerry
        pros::delay(2000);
        chassis.moveToPoint(-41.2, -1, 2000, { .forwards = false }, false); // jerry
        little_will_pnu.set_value(false);
        chassis.turnToHeading(-5, 1000);
        chassis.moveToPoint(-41.2, 15, 2000, {}, false);
        // line up to long-goal and score
        //chassis.turnToHeading(0, 1000);
        //chassis.moveToPoint(-41.8, 13.4, 2000, {}, false); // jerry
        // tower.scoreTop(1.0); // loud as shit

        // fly over to the other side for SAWP
        // chassis.moveToPoint(21.5, 9.4, 2000, { .forwards = false }); // jerry
        // chassis.moveToPose(0, 0, 0, 2000, { .forwards = false }); // jerry
}
