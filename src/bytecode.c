#include "bytecode.h"
#include "../include/pros/rtos.h"
#include "config.h"
#include "../include/pros/motors.h"
#include "../include/pros/adi.h"

void bc_execute(struct bytecode inst)
{
        switch (inst.type) {
        case BC_CYCLE:
                delay(ROBOT_TICK_DELAY);
                break;
        case BC_LEFTDRIVE:
                motor_move_voltage(MOTOR_CONFIG_L1.port, inst.value * (MOTOR_CONFIG_L1.reverse ? -1: 1));
                motor_move_voltage(MOTOR_CONFIG_L2.port, inst.value * (MOTOR_CONFIG_L2.reverse ? -1: 1));
                break;
        case BC_RIGHTDRIVE:
                motor_move_voltage(MOTOR_CONFIG_R1.port, inst.value * (MOTOR_CONFIG_R1.reverse ? -1: 1));
                motor_move_voltage(MOTOR_CONFIG_R2.port, inst.value * (MOTOR_CONFIG_R2.reverse ? -1: 1));
                break;
        case BC_BELT:
                motor_move_voltage(MOTOR_CONFIG_BELT.port, inst.value * (MOTOR_CONFIG_BELT.reverse ? -1: 1));
                break;
        case BC_INTAKE:
                motor_move_voltage(MOTOR_CONFIG_INTAKE.port, inst.value * (MOTOR_CONFIG_INTAKE.reverse ? -1: 1));
                break;
        case BC_SOLENOID:
                adi_port_set_config(SOLENOID_PORT, E_ADI_DIGITAL_OUT);
                adi_digital_write(SOLENOID_PORT, inst.value);
                break;
        }
}
