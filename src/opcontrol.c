#include "../include/main.h"
#include "../include/pros/rtos.h"
#include "bytecode.h"
#include "config.h"
#include "bytecode_stack.h"
#include "controls.h"

/*
 * An individual cycle during opcontrol
 */
void cycle(
        uint32_t tick,
        struct bc_stack_node **bytecode_stack,
        bool *solenoid_active,
        uint32_t *solenoid_tick
);

/*
 * The official prosv5 opcontrol entry point
 */
void opcontrol()
{
        /*
         * variables that get mutated
         */
        uint32_t now = millis(); /* the time of the last cycle */
        uint32_t tick = 0; /* how many cycles there have been */
        struct bc_stack_node *bytecode_stack = NULL; /* the bytecode stack */
        bool solenoid_active = false; /* if the solenoid is currently active */
        uint32_t solenoid_tick = 0; /* the last tick the solanoid was active */
        
        /*
         * opcontrol loop
         */
        while (true) {
                cycle(tick, &bytecode_stack, &solenoid_active, &solenoid_tick);
                task_delay_until(&now, ROBOT_TICK_DELAY);
        }
}

void cycle(uint32_t tick, struct bc_stack_node **bytecode_stack, bool *solenoid_active, uint32_t *solenoid_tick)
{
        /* get drive instructions */
        gen_drive_inst(bytecode_stack);

        /* execute all instructions on the bytecode stack */
        struct optional_bytecode inst = bc_stack_pop(bytecode_stack);
        while (inst.is_some) {
                bc_execute(inst.inst);
                inst = bc_stack_pop(bytecode_stack);
        }
}
