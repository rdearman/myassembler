/*
 A very simple program I can use to sanity check the machine code being generated.
 I can manually generate the correct machine code and validate the rust program
*/


start:
	shr r1
	shl r2
	bl loop
	// create a label further down in the program.
loop:
	bl start
	
