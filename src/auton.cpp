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
}

void autonRight() {
        // init stuff
        chassis.setPose(0, 0, 0);
        TowerState tower;
}
