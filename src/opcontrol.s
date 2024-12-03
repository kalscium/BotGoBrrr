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

	@ spins a motor at 12000 rpm
	mov r0, #1
	mov r1, #12000
	bl motor_move_voltage
	
	pop {lr}
	bx lr @ return to to the c function

.data
hw: .asciz "hello, world!"
random: .asciz "random number: "
