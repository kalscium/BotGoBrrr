#include "config.h"

const struct motor_config MOTOR_CONFIG_L1 = { 15, false };
const struct motor_config MOTOR_CONFIG_L2 = { 18, false };
const struct motor_config MOTOR_CONFIG_R1 = { 9, true };
const struct motor_config MOTOR_CONFIG_R2 = { 4, true };

const struct motor_config MOTOR_CONFIG_BELT = { 12, false };
const struct motor_config MOTOR_CONFIG_INTAKE = { 10, false };

const double ROBOT_TURN_SPEED = 0.64;
const int ROBOT_BELT_VOLTAGE = 760;
const double ROBOT_PRECISE_MULTIPLIER = 0.60;
const double ROBOT_DMN = 1.02022606038826; /* 12000 = 1024a^{127} */
