.global opcontrol

.text
opcontrol:
	@ push to the stack
	push {lr}

	@ print hello world
	ldr r0, =hw
	bl printf

	@ print a 'random' number
	ldr r0, =random
	mov r1, #7
	bl printf

	@ spins motor at port 6 at 12000 rpm
	mov r0, #6
	mov r1, #12000
	bl motor_move_voltage
	
	pop {lr}
	bx lr @ return to the c function

.data
hw: .asciz "hello, world!\n"
random: .asciz "random number: %d\n"
