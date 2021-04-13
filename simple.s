.data

$life_the_universe_and_everything .byte 42
$the_string .asciz "Hello World!"
$non_null_string .ascii "Not Terminated"

.text


start:
	PUSH {pc sp r1 r2}
	mov r1 #55
	mov r2 #2
	ldr r1 $life_the_universe_and_everything // load into r1 the value at memory address pointed to by
	ldr r1 [r4 #32]
	ldr r2 [r4 r1]
	ldr r3 [r4 #255]
	ldr r3 [r4 #0]
	str r1 $life_the_universe_and_everything
	str r3 [r4 #125]
	str r4 [r1 #0]
	str r1 [r4 #32]
	str r2 [r4 r1]
	str r3 [r4 #255]
	str r3 [r4 #0]
	add r2 r3
	BEQ start
	POP {pc sp r1 r2}

	shr r3
	shl r1
	add r1 r3
	bne next_one
	pop {pc sp r1 r2}
	
next_one:
	PUSH {r1 r2}
	mov r3 r4
	add r4 r1
	add r4 r2
	pop {r1 r2}
	bl start
