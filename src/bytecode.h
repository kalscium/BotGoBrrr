#ifndef BYTECODE_H_
#define BYTECODE_H_

/*
 * A single bytecode instruction type for the robot
 */
enum bytecode_type {
        CYCLE,
        LEFTDRIVE,
        RIGHTDRIVE,
        BELT,
        INTAKE,
        SOLENOID,
};

/*
 * A single complete bytecode instruction for the robot
 */
struct bytecode {
        enum bytecode_type type;
        int value;
};

/*
 * Executes a piece of bytecode and returns if the program should cycle or not
 */
bool execute(struct bytecode inst);

#endif
