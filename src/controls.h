#ifndef CONTROLS_H_
#define CONTROLS_H_

#include "bytecode.h"
#include "bytecode_stack.h"

/*
 * Generates bytecode instructions for driving from the current state of the controller
 */
void gen_drive_inst(struct bc_stack_node **bytecode_stack);

/*
 * Generates a solenoid instructions from the current state of the controller
 */
void gen_solenoid_inst(uint32_t tick, bool *solenoid_active, uint32_t *solenoid_tick, struct bc_stack_node **bytecode_stack);

/*
 * Generates belt and intake instructions from the current state of the controller
 */
void gen_belt_intake_inst(struct bc_stack_node **bytecode_stack);

#endif
