// Robot tower and pneumatics

#include "api.h"
#include "pros/adi.hpp"
#include "pros/misc.h"
#include "tower.hpp"

// The robot's middle roller
pros::MotorGroup tower_middle_intake(
        { 12, 13 },
        pros::MotorGearset::green
);

// The robot's intake roller

// The robot tower hood and storage motor
pros::MotorGroup tower_hood_storage(
    { 20, 6 },
    pros::MotorGearset::green
);

// The ADI port of the little will
pros::adi::DigitalOut little_will_pnu('A');

// The driver tower speed
double driverTowerSpeed = 1.0;
double driverTowerOutSpeed = 1.0;

// The ADI port of the park
pros::adi::DigitalOut park_pnu('H');

void TowerState::storeBlocks(double velocity) {
        tower_hood_storage.move_voltage((int) (-velocity * 12000));
        tower_middle_intake.move_voltage((int) (velocity * 12000));
}

void TowerState::scoreTop(double velocity) {
        tower_hood_storage.move_voltage((int) (velocity * 12000));
        tower_middle_intake.move_voltage((int) (velocity * 12000));
}

void TowerState::scoreBottom(double velocity) {
        tower_hood_storage.move_voltage((int) (velocity * 12000));
        tower_middle_intake.move_voltage((int) (-velocity * 12000));
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
                scoreTop(driverTowerSpeed);
        } else if (master.get_digital(pros::E_CONTROLLER_DIGITAL_R1)) {
                // outtake
                scoreBottom(driverTowerOutSpeed);
                intake = false;
        } else if (intake) {
                // store
                storeBlocks(driverTowerSpeed);
        } else {
                scoreTop(0.0);
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
