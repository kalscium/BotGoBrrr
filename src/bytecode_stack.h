#ifndef BYTECODE_STACK_H_
#define BYTECODE_STACK_H_

#include "bytecode.h"

/*
 * An optional bytecode instruction that may or may not exist
 */
struct optional_bytecode {
        bool is_some;
        struct bytecode inst;
};

/*
 * A single node in the bytecode stack (singlely linked list)
 */
struct bc_stack_node {
        struct bytecode inst;
        struct bc_stack_node *next;
};

/*
 * Initialises a new bytecode stack with an initial value
 */
struct bc_stack_node *bc_stack_init(struct bytecode inst);

/*
 * Pushes a new instruction onto the bytecode stack
 */
void bc_stack_push(struct bc_stack_node **head_ref, struct bytecode inst);

/*
 * Pops an instruction off the bytecode stack
 * 
 * May return nothing if the bytecode stack is empty
 */
struct optional_bytecode bc_stack_pop(struct bc_stack_node **head_ref);

/*
 * Frees the heap memory used by the bytecode stack
 *
 * Can be also used to wipe the bytecode stack
 */
void bc_stack_free(struct bc_stack_node **head_ref);

#endif
