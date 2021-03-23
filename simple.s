start:
	shr r1
	bne bottom

next:
	shl r2
	shl r3
	shl r4
	bl start

bottom:
	bl start