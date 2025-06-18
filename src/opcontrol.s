.global asm_opcontrol
.extern printf controller_get_analog motor_move task_delay_until millis

.equ tick_delay, 10

.equ drive_motor_l1, 12
.equ drive_motor_l2, 12
.equ drive_motor_l3, 12
.equ drive_motor_r1, 12
.equ drive_motor_r2, 12
.equ drive_motor_r3, 12

.text
asm_opcontrol:
        @ standard function boiler
        push {fp, lr}
        add fp, sp, #4

        @ allocate a 'now' variable on the stack
        bl millis @ sets r0 to current time
        str r0, [sp, #-4]! @ store on the stack

        @ say hi, to prove everything is running
        ldr r0, =message
        bl printf

        b loop

        @ return (unreachable, but again, feels nice and standard)
        sub sp, fp, #4
        pop {fp, lr}
        bx lr
@ main opcontrol loop (forever)
loop:
        @ print message of hope
        ldr r0, =hope
        bl printf

        @ store the left joystick y value in r4
        mov r0, #0 @ set CONTROLLER_MASTER id
        mov r1, #1 @ set CONTROLLER_ANALOG_LEFT_Y channel
        bl controller_get_analog
        mov r4, r0

        @ store the right joystick y value in r5
        mov r0, #0 @ set CONTROLLER_MASTER id
        mov r1, #3 @ set CONTROLLER_ANALOG_RIGHT_Y channel
        bl controller_get_analog
        mov r5, r0

        @ drive
        bl drive

        @ wait for the next cycle
        sub r0, fp, #4
        mov r1, #tick_delay

        b loop
@ drive function
drive:
        push {lr}

        @ motor l1
        mov r0, #drive_motor_l1
        mov r1, r4 @ should be stored in r4 
        bl motor_move
        @ motor l2
        mov r0, #drive_motor_l2
        mov r1, r4 @ should be stored in r4 
        bl motor_move
        @ motor l3
        mov r0, #drive_motor_l3
        mov r1, r4 @ should be stored in r4 
        bl motor_move

        @ motor r1
        mov r0, #drive_motor_r1
        mov r1, r5 @ should be stored in r5 
        bl motor_move
        @ motor r2
        mov r0, #drive_motor_r2
        mov r1, r5 @ should be stored in r5 
        bl motor_move
        @ motor r3
        mov r0, #drive_motor_r3
        mov r1, r5 @ should be stored in r5 
        bl motor_move

        pop {lr}
        bx lr

.data
message: .asciz "hello, world from arm-assembly\n"
hope: .asciz "not segfaulted yet!\n"
