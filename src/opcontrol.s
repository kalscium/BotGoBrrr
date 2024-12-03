.global opcontrol

.text
opcontrol:
	ldr r0, =msg
	bl printf
	
	bx lr @ return to to the c function

.data
msg: .asciz "hello, world!"
