#include "config.h"

const uint32_t ROBOT_TICK_DELAY = 50;
// const uint32_t ROBOT_TICK_DELAY = 1000; /* for testing purposes */

const struct motor_config MOTOR_CONFIG_L1 = { 16, false };
const struct motor_config MOTOR_CONFIG_L2 = { 19, false };
const struct motor_config MOTOR_CONFIG_L3 = { 18, false };
const struct motor_config MOTOR_CONFIG_R1 = { 9, true };
const struct motor_config MOTOR_CONFIG_R2 = { 4, true };
const struct motor_config MOTOR_CONFIG_R3 = { 5, true };

const struct motor_config MOTOR_CONFIG_BELT = { 12, false };
const struct motor_config MOTOR_CONFIG_INTAKE = { 10, false };

const double ROBOT_TURN_SPEED = 0.64;
const int32_t ROBOT_BELT_VOLTAGE = 7600;
const double ROBOT_PRECISE_MULTIPLIER = 0.60;
const double ROBOT_DMN = 1.02022606038826; /* 12000 = 1024a^{127} */

const uint8_t SOLENOID_PORT = 1;
const uint32_t SOLENOID_DELAY = 8;

