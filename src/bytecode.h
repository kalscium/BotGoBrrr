#ifndef BYTECODE_H_
#define BYTECODE_H_

#include <stdbool.h>
#include <stdint.h>

/*
 * A single bytecode instruction type for the robot
 */
enum bytecode_type {
        BC_CYCLE,
        BC_LEFTDRIVE,
        BC_RIGHTDRIVE,
        BC_BELT,
        BC_INTAKE,
        BC_SOLENOID,
};

/*
 * A single complete bytecode instruction for the robot
 */
struct bytecode {
        enum bytecode_type type;
        int32_t value;
};

/*
 * Executes a bytecode instruction
 */
void bc_execute(struct bytecode inst);

#endif
