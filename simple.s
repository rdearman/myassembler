start:
	mov r1 #55
	mov r2 #2
	mov r3 r1
	sub r1 r2
	add r2 r3
	BEQ start

	shr r3
	shl r1
	add r1 r3
	bne start

	mov r3 r4
	add r4 r1
	add r4 r2
	
