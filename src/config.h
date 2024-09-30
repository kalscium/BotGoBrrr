#ifndef CONFIG_H_
#define CONFIG_H_

#include <stdint.h>
#include <stdbool.h>

/*
 * An individual configuration for a motor 
 */
struct motor_config {
        uint8_t port;
        bool reverse;
};

/*
 * How long a single robot bytecode execution cycle 'tick'/delay is in milliseconds (1000ms in 1s)
 */
extern const uint32_t ROBOT_TICK_DELAY;

/*
 * Motor configurations for the main four drive motors
 */
extern const struct motor_config MOTOR_CONFIG_L1;
extern const struct motor_config MOTOR_CONFIG_L2;
extern const struct motor_config MOTOR_CONFIG_R1;
extern const struct motor_config MOTOR_CONFIG_R2;

/*
 * Motor configurations for the belt and intake
 */
extern const struct motor_config MOTOR_CONFIG_BELT;
extern const struct motor_config MOTOR_CONFIG_INTAKE;

/*
 * The robot's turning speed (as a multiplier)
 */
extern const double ROBOT_TURN_SPEED;

/*
 * The robot's conveyor belt voltage out of `12000`
 */
extern const int32_t ROBOT_BELT_VOLTAGE;

/*
 * The mulitplier for the robot's precise movement speed
 */
extern const double ROBOT_PRECISE_MULTIPLIER;

/*
 * Daniel's magic number for the joysticks
 */
extern const double ROBOT_DMN;

/*
 * The adi port of the pneumatics solanoid and the tick delay of that solanoid
 * 
 * Used for stopping solanoid from just turning on and off every tick
 */

#endif
