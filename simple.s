.data

$life_the_universe_and_everything .byte 42
$the_string .asciz "Hello World!"
$non_null_string .ascii "Not Terminated"

.text


start:
	mov r1 #55
	mov r2 #2
	ldr r3 $life_the_universe_and_everything
	str r1 [r3]
	sub r1 r2
	add r2 r3
	BEQ start

	shr r3
	shl r1
	add r1 r3
	bne next_one

next_one:
	mov r3 r4
	add r4 r1
	add r4 r2
	bl start
