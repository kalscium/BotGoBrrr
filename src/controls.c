#include "controls.h"
#include <math.h>
#include <stdint.h>
#include <stdlib.h>
#include "../include/pros/misc.h"
#include "bytecode.h"
#include "bytecode_stack.h"
#include "config.h"

double clamp(double x, double min, double max)
{
        if (x < min)
                return min;
        else if (x > max)
                return max;
        else
                return x;
}

void gen_drive_inst(struct bc_stack_node **bytecode_stack)
{
        /* get joystick values */
        int32_t j1x = controller_get_analog(E_CONTROLLER_MASTER, E_CONTROLLER_ANALOG_LEFT_X);
        int32_t j1y = controller_get_analog(E_CONTROLLER_MASTER, E_CONTROLLER_ANALOG_LEFT_Y);

        /* get the calculated voltage for the x of j1 */
        double j1xv = (1024.0 * pow(ROBOT_DMN, (double) abs(j1x)) - 1024.0)
                * j1x < 0 ? -1.0: 1.0 /* unabsolute the numbers */
                * ROBOT_TURN_SPEED; /* reduce turning speed */

        /* get the calculated voltage for the y of j1 */
        double j1yv = (1024.0 * pow(ROBOT_DMN, (double) abs(j1y)) - 1024.0) * j1y < 0 ? -1.0: 1.0;

        /* reduce the voltages / speeds of the motors if precise driving is on */
        if (controller_get_digital(E_CONTROLLER_MASTER, E_CONTROLLER_DIGITAL_L2)) {
                j1xv *= ROBOT_PRECISE_MULTIPLIER;
                j1yv *= ROBOT_PRECISE_MULTIPLIER;
        }

        /* calculate the left and right drives according to arcade controls */
        double ldr = clamp(j1yv + j1xv, -12000.0, 12000.0);
        double rdr = clamp(j1yv - j1xv, -12000.0, 12000.0);

        /* swap the left and right drives and flip the sign if the robot is driving in reversed mode */
        if (controller_get_digital(E_CONTROLLER_MASTER, E_CONTROLLER_DIGITAL_L1)) {
                ldr -= rdr;
                rdr += ldr;
                ldr = rdr - ldr;

                ldr = -ldr;
                rdr = -rdr;
        }

        /* push the left and right drive bytecode instructions */
        struct bytecode ldri = { BC_LEFTDRIVE, (int32_t) ldr };
        struct bytecode rdri = { BC_LEFTDRIVE, (int32_t) rdr };
        bc_stack_push(bytecode_stack, ldri);
        bc_stack_push(bytecode_stack, rdri);
}
