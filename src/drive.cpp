// Drivetrian stuff

#include "drive.hpp"
#include "api.h"
#include "lemlib/api.hpp"

// Left Drivetrain Motors
pros::MotorGroup right_dt({ 10, 9, -14 }, pros::MotorGearset::blue);
// Right Drivetrain Motors
pros::MotorGroup left_dt({ -11, -2, 12 }, pros::MotorGearset::blue);

// The track-width in mm
double track_width_mm = 290;

// The Lemlib Drivetrain
lemlib::Drivetrain drivetrain(&left_dt, &right_dt, track_width_mm / 25.4, lemlib::Omniwheel::NEW_325, 450, 2);

// The IMU port
pros::Imu imu(15);

// The odom port
pros::Rotation lateral_rotation(16);
lemlib::TrackingWheel lateral_trackwh(&lateral_rotation, lemlib::Omniwheel::NEW_275, 0);

// The odom configs
lemlib::OdomSensors odom_sensors(&lateral_trackwh, nullptr, nullptr, nullptr, &imu);

// lateral PID controller
lemlib::ControllerSettings lateral_controller(
        10, // proportional gain (kP)
        0, // integral gain (kI)
        3, // derivative gain (kD)
        3, // anti windup
        1, // small error range, in inches
        100, // small error range timeout, in milliseconds
        3, // large error range, in inches
        500, // large error range timeout, in milliseconds
        20 // maximum acceleration (slew)
);

// angular PID controller
lemlib::ControllerSettings angular_controller(
        2, // proportional gain (kP)
        0, // integral gain (kI)
        10, // derivative gain (kD)
        3, // anti windup
        1, // small error range, in degrees
        100, // small error range timeout, in milliseconds
        3, // large error range, in degrees
        500, // large error range timeout, in milliseconds
        0 // maximum acceleration (slew)
);

// create the chassis
lemlib::Chassis chassis(
        drivetrain,
        lateral_controller,
        angular_controller,
        odom_sensors
);

void initDt() {
        chassis.calibrate();
}

void driverDrive() {
        while (true) {
                pros::Controller master(pros::E_CONTROLLER_MASTER);
                int throttle = master.get_analog(pros::E_CONTROLLER_ANALOG_LEFT_Y);
                int steer = master.get_analog(pros::E_CONTROLLER_ANALOG_RIGHT_X);  

                // chassis.arcade(throttle, steer, false, 0.75);
                chassis.curvature(throttle, steer);

                pros::delay(20);
        }
}
