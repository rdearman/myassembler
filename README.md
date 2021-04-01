# myassembler

This is a very super hacky attempt at creation of an assembler for my 8-bit breadboard CPU. It is primarily an exercise in learning more about electronics and the rust programming language. It should in no way be considered useful for anyone who doesn't own my homemade CPU.

I've designed the assembly language to be like GAS syntax, but it doesn't support most things.

The plan is to support the following:

 <pre>
/* ------------- My Assembler Language ----------------
ADD reg reg | arg
SUB reg reg | arg
AND reg reg
ORR reg reg
XOR reg reg
NOT reg
CMP reg arg
MOV reg, reg | arg
LDR reg mem
STR reg mem
SHR reg
SHL reg
INC reg
DEC reg
CCF (clear carry flag)
BL  [copy PC to SP and branch]
BEQ [Branch Equal]
BNE [Branch Not Equal]
BLT [Branch Less Than]
BGT [Branch Greater Than]
PUSH
POP

label:

_variable_name .asciz " +"
_variable_name .byte 0
 </pre>