[] create functions for all instructions
[] make small asm program which will test all instructions
[] support labels further down in file (may need 2 step lex, first for labels and data)
[] support the _data label. the underscore indicated variable name.
[] way to separate program memory from storage memor (probably hardware not software)


[X] import bytecode to generate instructions.
[X] first two bytecodes are always the same, make the constructor default: 
    load PC into MAR
    load MDR into IR
[X] define all descreet bytecodes for use in instructions. (bindgen)