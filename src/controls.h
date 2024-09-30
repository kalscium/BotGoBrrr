#ifndef CONTROLS_H_
#define CONTROLS_H_

#include "bytecode_stack.h"

/*
 * Generates bytecode instructions for driving from the current state of the controller
 */
void gen_drive_inst(struct bc_stack_node **bytecode_stack);

#endif
