#include "../include/main.h"
#include "../include/pros/rtos.h"
#include "config.h"

/*
 * An individual cycle during opcontrol
 */
void cycle(
        uint32_t tick,
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
        bool solenoid_active = false; /* if the solenoid is currently active */
        uint32_t solenoid_tick = 0; /* the last tick the solanoid was active */
        
        /*
         * opcontrol loop
         */
        while (true) {
                cycle(tick, &solenoid_active, &solenoid_tick);
                task_delay_until(&now, ROBOT_TICK_DELAY);
        }
}

void cycle(uint32_t tick, bool *solenoid_active, uint32_t *solenoid_tick) {
        
}
