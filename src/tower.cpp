// Robot tower and pneumatics

#include "api.h"
#include "pros/adi.hpp"
#include "pros/misc.h"
#include "tower.hpp"

// The robot tower rollers (everything that's not the hood)
pros::MotorGroup tower_rollers(
        { -5, 16 },
        pros::MotorGearset::green
);

// The robot tower hood
pros::Motor tower_hood(-6, pros::MotorGearset::green);

// The ADI port of the little will
pros::adi::DigitalOut little_will_pnu('A');

// The driver tower speed
double driverTowerSpeed = 1.0;
double driverTowerOutSpeed = 1.0;

// The ADI port of the park
pros::adi::DigitalOut park_pnu('H');

void TowerState::storeBlocks(double velocity) {
        tower_hood.move_velocity(-velocity);
        tower_rollers.move_velocity(velocity);
}

void TowerState::spin(double velocity) {
        tower_hood.move_velocity(velocity);
        tower_rollers.move_velocity(velocity);
}

void TowerState::controls() {
        pros::Controller master(pros::E_CONTROLLER_MASTER);

        // check for the intake toggle
        if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_R2)) {
                intake = !intake;
                if (intake) // rumble when down
                        master.rumble(".");
        }

        // check for scoring
        if (master.get_digital(pros::E_CONTROLLER_DIGITAL_R2) && master.get_digital(pros::E_CONTROLLER_DIGITAL_R1)) {
                intake = false;
                spin(driverTowerSpeed);
        } else if (master.get_digital(pros::E_CONTROLLER_DIGITAL_R1)) {
                // outtake
                spin(-driverTowerOutSpeed);
                intake = false;
        } else if (intake) {
                // store
                storeBlocks(driverTowerSpeed);
        } else {
                spin(0.0);
        }

        // check for park
        if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_Y)) {
                park = !park;
                // rumble if down
                if (park)
                        master.rumble(".-");
                park_pnu.set_value(park);
        }

        // little will toggle
        if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_B)) {
                little_will = !little_will;
                // rumble if down
                if (little_will)
                        master.rumble("-");
                little_will_pnu.set_value(little_will);
        }
}
