// Robot tower and pneumatics

#include "api.h"
#include "pros/abstract_motor.hpp"
#include "pros/adi.hpp"
#include "pros/error.h"
#include "pros/misc.h"
#include "tower.hpp"

// The robot's intake roller
pros::Motor tower_intake(
        -14,
        pros::MotorGearset::green
);

// The port of the optical sensor
pros::Optical optical(0);

// The ADI port of the little will
pros::adi::DigitalOut little_will_pnu('E');

// The ADI port of the snacky
pros::adi::DigitalOut snacky_pnu('F');

// The ADI port of the 'barrel'
pros::adi::DigitalOut barrel_pnu('A');

// The ADI port of the 'gate'
pros::adi::DigitalOut gate_pnu('B');

// The lever motor port
pros::Motor lever_motor(
        -20,
        pros::MotorGearset::green
);
// The lever rotation sensor port
pros::Rotation lever_rotation(19);

// The tower outtake speed
double outtake_speed = 0.5;

// Lever scoring middle goal speed
double lever_middle_speed = 0.5;

void TowerState::spinIntake(double velocity) {
        tower_intake.move_voltage((int) (velocity * 12000));
}

void TowerState::moveLever(double velocity) {
        lever_motor.move_velocity(velocity * 200);
}

void initTower() {
        lever_rotation.reset_position();
}

// check if the lever is in bounds, if not, stop it
void TowerState::leverBoundCheck() {
        int32_t rotation = lever_rotation.get_position();
        // check if the lever's toggled backwards or forwards
        if (!lever_active && rotation < 4000 || lever_active && rotation > 40000) { // toggled on and already in the back
                // stop the lever
                moveLever(0);
        }
}

bool red_checkBlue() {
        // blue is really simple to test for (simple range)
        return optical.get_hue() > 210 and optical.get_hue() < 300 and optical.get_proximity() > 200;
}

bool blue_checkRed() {
        // red is complicated since it wraps around <200 -> 350
        return (optical.get_hue() < 200 or optical.get_hue() > 300) and optical.get_proximity() > 200;
}

void TowerState::colorSort(double velocity) {
        // optical.set_led_pwm(100);
        // if (use_color_sort)
        // if (red_checkBlue()) // when we are RED
        // // if (blue_checkRed()) // when we are BLUE
        //         time_since_optic = pros::rtos::millis();

        // if (pros::rtos::millis() - time_since_optic < 160)
        //         tower_storage.move_voltage((int) (velocity * 12000));
        // else if (pros::rtos::millis() - time_since_optic < 225)
        //         tower_storage.move_voltage((int) (-velocity * 12000));
        // else
        //         tower_storage.move_voltage((int) (-velocity * 0));
}

void opticTest() {
        while (true) {
                printf("hue: %lf, proximity: %lf\n", optical.get_hue(), optical.get_proximity());
                pros::delay(50);
        }
}

void TowerState::controls() {
        pros::Controller master(pros::E_CONTROLLER_MASTER);

        // check for the intake toggle and out-take hold
        if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_DOWN)) {
                intake = !intake;
                if (intake) { // rumble when active
                        master.rumble(".");
                        spinIntake(1.0);
                } else {
                        spinIntake(0.0);
                }
        } else if (master.get_digital(pros::E_CONTROLLER_DIGITAL_UP)) {
                spinIntake(-outtake_speed); // otherwise out-take at a slow speed
                intake = false;
        }
        if (master.get_digital_new_release(pros::E_CONTROLLER_DIGITAL_UP)) {
                spinIntake(0.0);
                intake = false;
        }


        // check for scoring with lever toggle (top and middle)
        if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_R2)) {
                lever_active = !lever_active;
                if (lever_active) {
                        gate_pnu.set_value(true);
                        if (score_top)
                                moveLever(1.0);
                        else
                                moveLever(0.5);
                } else {
                        gate_pnu.set_value(false);
                        moveLever(-0.5);
                }
        }
        // lever bounds check
        leverBoundCheck();

        // barrel toggle (score top or score mid)
        if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_R1) && !lever_active) {
                score_top = !score_top;
                barrel_pnu.set_value(score_top);
                if (score_top) // rumble on barrel scoring top
                        master.rumble(".");
        }

        // // panic disable color sort
        // if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_UP))
        //         use_color_sort = false;

        // little will toggle
        if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_B)) {
                little_will = !little_will;
                // rumble if down
                if (little_will)
                        master.rumble("-");
                little_will_pnu.set_value(little_will);
        }

        // snacky toggle
        if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_A)) {
                snacky = !snacky;
                // rumble if down
                if (snacky)
                        master.rumble("-");
                snacky_pnu.set_value(snacky);
        }
}
