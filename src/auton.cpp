#include "auton.hpp"
#include "drive.hpp"
#include "api.h"
#include "lemlib/api.hpp"

extern lemlib::Chassis chassis;

void autonLeft() {
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
