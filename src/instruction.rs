// Instruction Manager
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use crate::bindings::eOpcodes;
use crate::my_operation::{Label, OperationalCode, Unresolved};
use regex::Regex;

pub fn def_pop(elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    //println!("elem={:?} \n",elem );
    let strip = Regex::new(r"[[:space:]]+pop[[:space:]]*?[\{]+(.*)[\}]+").unwrap();
    let rxmany = Regex::new(r"([[:word:]]+)").unwrap();
    let stripped_variable = &strip.captures(&elem).unwrap()[1];
    //println!("{:?}\n", stripped_variable);
    let immediate_count = &rxmany.captures_iter(&elem).count();
    //println!("COUNT: {:?}\n", immediate_count);
    if immediate_count > &5 // I only have 5 bytecode instructions! Might be restricted to two ?
    {
        panic!("Cannot POP more than 4 registers in {:?}", &elem);
    }

    let marray = rxmany.captures_iter(stripped_variable);

    // for element in marray
    // {
    //     // OK push each of these on the stack
    //     let fred = element.get(1).unwrap().as_str();
    //     println!("{:?}\n", fred);
    // }


    let another_opcode: OperationalCode = OperationalCode::new(
        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
        crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
    );
    opcodes_vector.push(another_opcode);
    return binloc + 1 as u16;

}

pub fn def_push(elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    //println!("elem={:?} \n",elem );
    let strip = Regex::new(r"[[:space:]]*?push[[:space:]]*?[\{]+(.*)[\}]+").unwrap();
    let rxmany = Regex::new(r"([[:word:]]+)").unwrap();
    let stripped_variable = &strip.captures(&elem).unwrap()[1];
    //println!("{:?}\n", stripped_variable);
    let immediate_count = &rxmany.captures_iter(&elem).count();
    if immediate_count > &5 // I only have 5 bytecode instructions! Might be restricted to two ?
    {
        panic!("Cannot PUSH more than 4 registers in {:?}", &elem);
    }

    let marray = rxmany.captures_iter(stripped_variable);

    // for element in marray
    // {
    //     // OK push each of these on the stack
    //     let fred = element.get(1).unwrap().as_str();
    //     println!("{:?}\n", fred); 
    // }


    let another_opcode: OperationalCode = OperationalCode::new(
        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
        crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
    );
    opcodes_vector.push(another_opcode);
    return binloc + 1 as u16;

}

pub fn def_store_memory(instr: i32, elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    //println!("elem={:?} \n",elem );
    /*
        ISSUES
        1 There is a problem here when the offset is a register. So need to check for the # character and assume it 
            is a register as the offset otherwise.
        2 This is quite a complex operation since it will need to load the value of the register. might be more than 
            5 microcodes, which is the limit. (Might be able to change the HW to support more than 7 timers total)
        3 A variable can be either a pointer to a string, or a byte (number), so need to work out which it is. 
        4 It is also possible fr the programmer to just want to load the memory adress pointed at by another register 
            e.g. "LDR r1 [r4]"
    */
    match instr {
        0 => { // Load Variable (issue 3)
            let rx = Regex::new(r"[[:space:]]+str[[:space:]]+([[:word:]]+)[[:space:]]+\$*-?[[:word:]]+").unwrap();
            let rxvariable = Regex::new(r".*[[:space:]]+(\$*-?[[:word:]]+)").unwrap();
            let matched_register = &rx.captures(&elem).unwrap()[1];
            let matched_variable = &rxvariable.captures(&elem).unwrap()[1];
            //println!("matched_register={:?} \t matched_variable={:?}\n",matched_register, matched_variable );
        }
        1 => { // Load Memory location + Offest (issue 1 & 2)
            let rx = Regex::new(r"[[:space:]]+str[[:space:]]+([[:word:]]+)[[:space:]]+[\[]+.*").unwrap();
            let rxlocation = Regex::new(r".*[\[]+[[:space:]]*?([[:word:]]+).*").unwrap();
            let rxoffset = Regex::new(r".*[\[]+[[:space:]]*?[[:word:]]+[[:space:]]*?([[:word:]]+)[\]]+").unwrap();
            // offset can be a register or an immedaite value. Check if immedaite first.
            let rxdigit = Regex::new(r".*[[:space:]]#(-?[[:digit:]]+)").unwrap();
            let immediate_count = &rxdigit.captures_iter(&elem).count();
            let mut use_immediate_value: bool = false;
            let mut rxdigitvalue: i32 = 0;
            let mut matched_offset: &str = "string";

            match immediate_count {
                1 => {
                    rxdigitvalue = rxdigit.captures(&elem).unwrap()[1]
                        .parse::<i32>()
                        .unwrap_or_default();
                    if rxdigitvalue > 255 {
                        panic!("Immediate value greater than 255 at: {:?}", elem);
                    }
                    use_immediate_value = true;
                }
                _ => {
                    matched_offset = &rxoffset.captures(&elem).unwrap().get(1).unwrap().as_str();
                }
            }

            let matched_register = &rx.captures(&elem).unwrap()[1];
            let matched_variable = &rxlocation.captures(&elem).unwrap()[1];
            // if use_immediate_value
            // {
            //     println!("matched_register={:?} \t matched_variable={:?} \t matched_offset={:?}\n",matched_register, matched_variable, rxdigitvalue );
            // }
            // else {
            //     println!("matched_register={:?} \t matched_variable={:?} \t matched_offset={:?}\n",matched_register, matched_variable, matched_offset );
            // }

        }
        _ => {
            // panic!("Problem with \"{:?}\" in function def_load_memory", elem);
        }
    }


    let another_opcode: OperationalCode = OperationalCode::new(
        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
        crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
    );
    opcodes_vector.push(another_opcode);
    return binloc + 1 as u16;

}

pub fn def_load_memory(instr: i32, elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    //println!("elem={:?} \n",elem );
    /*
        ISSUES
        1 There is a problem here when the offset is a register. So need to check for the # character and assume it 
            is a register as the offset otherwise.
        2 This is quite a complex operation since it will need to load the value of the register. might be more than 
            5 microcodes, which is the limit. (Might be able to change the HW to support more than 7 timers total)
        3 A variable can be either a pointer to a string, or a byte (number), so need to work out which it is. 
        4 It is also possible fr the programmer to just want to load the memory adress pointed at by another register 
            e.g. "LDR r1 [r4]" (This case they will need to put #0 and offset of zero)
    */
    match instr {
        0 => { // Load Variable (issue 3)
            let rx = Regex::new(r"[[:space:]]+ldr[[:space:]]+([[:word:]]+)[[:space:]]+\$*-?[[:word:]]+").unwrap();
            let rxvariable = Regex::new(r".*[[:space:]]+(\$*-?[[:word:]]+)").unwrap();
            let matched_register = &rx.captures(&elem).unwrap()[1];
            let matched_variable = &rxvariable.captures(&elem).unwrap()[1];
            //println!("matched_register={:?} \t matched_variable={:?}\n",matched_register, matched_variable );
        }
        1 => { // Load Memory location + Offest (issue 1 & 2)
            let rx = Regex::new(r"[[:space:]]+ldr[[:space:]]+([[:word:]]+)[[:space:]]+[\[]+.*").unwrap();
            let rxlocation = Regex::new(r".*[\[]+[[:space:]]*?([[:word:]]+).*").unwrap();
            let rxoffset = Regex::new(r".*[\[]+[[:space:]]*?[[:word:]]+[[:space:]]*?([[:word:]]+)[\]]+").unwrap();
            // offset can be a register or an immedaite value. Check if immedaite first.
            let rxdigit = Regex::new(r".*[[:space:]]#(-?[[:digit:]]+)").unwrap();
            let immediate_count = &rxdigit.captures_iter(&elem).count();
            let mut use_immediate_value: bool = false;
            let mut rxdigitvalue: i32 = 0;
            let mut matched_offset: &str = "string";

            match immediate_count {
                1 => {
                    rxdigitvalue = rxdigit.captures(&elem).unwrap()[1]
                        .parse::<i32>()
                        .unwrap_or_default();
                    if rxdigitvalue > 255 {
                        panic!("Immediate value greater than 255 at: {:?}", elem);
                    }
                    use_immediate_value = true;
                }
                _ => {
                    matched_offset = &rxoffset.captures(&elem).unwrap().get(1).unwrap().as_str();
                }
            }

            let matched_register = &rx.captures(&elem).unwrap()[1];
            let matched_variable = &rxlocation.captures(&elem).unwrap()[1];
            // if use_immediate_value
            // {
            //     println!("matched_register={:?} \t matched_variable={:?} \t matched_offset={:?}\n",matched_register, matched_variable, rxdigitvalue );
            // }
            // else {
            //     println!("matched_register={:?} \t matched_variable={:?} \t matched_offset={:?}\n",matched_register, matched_variable, matched_offset );
            // }

        }
        _ => {
            // panic!("Problem with \"{:?}\" in function def_load_memory", elem);
        }
    }


    let another_opcode: OperationalCode = OperationalCode::new(
        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
        crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
    );
    opcodes_vector.push(another_opcode);
    return binloc + 1 as u16;

}
// add, sub, mov are almost carbon copies and really should be moved into one unified function. 
pub fn def_inc_dec(instr: i32, elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    // println!("ORIGINAL STRING: {:?}", elem);
    let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
    let matched_string = &rx.captures(&elem).unwrap()[1];

    let another_opcode: OperationalCode = OperationalCode::new(
        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
        crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
    );
    opcodes_vector.push(another_opcode);
    binloc += 1 as u16;

    match instr {
        0 => { // INC
            match matched_string {
                "r1" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_inc_r1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_inc_r2,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_inc_r3,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_inc_r4,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "pc" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_inc_pc,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "sp" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_inc_sp,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        },
        1 => { // DEC
            match matched_string {
                "r1" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_dec_r1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_dec_r2,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_dec_r3,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_dec_r4,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "sp" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_dec_sp,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        },
        _ => {
            panic!("Error with statement: {:?}", elem);
        }
    }

}

pub fn def_logic(instr: i32, elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    // println!("ORIGINAL STRING: {:?}", elem);
    let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
    let matched_string = &rx.captures(&elem).unwrap()[1];

    let another_opcode: OperationalCode = OperationalCode::new(
        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
        crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
    );
    opcodes_vector.push(another_opcode);
    binloc += 1 as u16;

    match instr {
        0 => { // NOT
            match matched_string {
                "r1" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_not_r1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_not_r2,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_not_r3,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_not_r4,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        },
        1 => { // XOR
            match matched_string {
                "r1" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_xor_r1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_xor_r2,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_xor_r3,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_xor_r4,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        },
        2 => { // AND
            match matched_string {
                "r1" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_and_r1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_and_r2,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_and_r3,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_and_r4,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        },
        3 => { // ORR
            match matched_string {
                "r1" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_or_r1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_or_r2,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_or_r3,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_or_r4,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }

        },
        _ => {
            panic!("Error with statement: {:?}", elem);
        }
    }

}

pub fn def_sub(elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    // println!("ORIGINAL STRING: {:?}", elem);
    let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
    let rxdigit = Regex::new(r".*[[:space:]]#(-?[[:digit:]]+)").unwrap();
    let matched_string = &rx.captures(&elem).unwrap()[1];
    //let second_matched_string = &rx.captures(&elem).unwrap()[2];
    let immediate_count = &rxdigit.captures_iter(&elem).count();
    let mut use_immediate_value: bool = false;
    let mut rxdigitvalue: i32 = 0;

    match immediate_count {
        1 => {
            rxdigitvalue = rxdigit.captures(&elem).unwrap()[1]
                .parse::<i32>()
                .unwrap_or_default();
            if rxdigitvalue > 255 {
                panic!("Immediate value greater than 255 at: {:?}", elem);
            }
            use_immediate_value = true;
        }
        _ => {
            //println!("immediate_count: {:?} ", immediate_count);
        }
    }

    /* Everything below is copied from the mov functions. Need to change the opcodes which it uses 
        and some of the logic. 
    */

    match use_immediate_value {
        true => {
            let another_opcode: OperationalCode = OperationalCode::new(
                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
            );
            opcodes_vector.push(another_opcode);
            binloc += 1 as u16;
            let immediate_address: OperationalCode = OperationalCode::new(
                binloc + 1 + crate::bindings::eOpcodes_opcode_Timer_3, rxdigitvalue as u16,
            );
            opcodes_vector.push(immediate_address);
            binloc += 1 as u16;
            match matched_string {
                "r1" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_sub_r1_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_sub_r2_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_sub_r3_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_sub_r4_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        }
        false => {
            // Then it has matched the second register, and we need to get the first register
            let rx =
                Regex::new(r".*?[[:space:]][[:word:]]+.*?[[:space:]]([[:word:]]+).*?").unwrap();
            let other_matched_string = &rx.captures(&elem).unwrap()[1];

            match matched_string {
                "r1" => {
                    match other_matched_string {
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r1_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r1_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r1_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r2" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r2_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r2_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r2_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r3" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r3_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r3_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r3_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r4" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r4_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r4_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_sub_r4_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        }
    }
}

pub fn def_add(elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    // println!("ORIGINAL STRING: {:?}", elem);
    let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
    let rxdigit = Regex::new(r".*[[:space:]]#(-?[[:digit:]]+)").unwrap();
    let matched_string = &rx.captures(&elem).unwrap()[1];
    //let second_matched_string = &rx.captures(&elem).unwrap()[2];
    let immediate_count = &rxdigit.captures_iter(&elem).count();
    let mut use_immediate_value: bool = false;
    let mut rxdigitvalue: i32 = 0;

    match immediate_count {
        1 => {
            rxdigitvalue = rxdigit.captures(&elem).unwrap()[1]
                .parse::<i32>()
                .unwrap_or_default();
            if rxdigitvalue > 255 {
                panic!("Immediate value greater than 255 at: {:?}", elem);
            }
            use_immediate_value = true;
        }
        _ => {
            //println!("immediate_count: {:?} ", immediate_count);
        }
    }

    /* Everything below is copied from the mov functions. Need to change the opcodes which it uses 
        and some of the logic. 
    */

    match use_immediate_value {
        true => {
            let another_opcode: OperationalCode = OperationalCode::new(
                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
            );
            opcodes_vector.push(another_opcode);
            binloc += 1 as u16;
            let immediate_address: OperationalCode = OperationalCode::new(
                binloc + 1 + crate::bindings::eOpcodes_opcode_Timer_3, rxdigitvalue as u16,
            );
            opcodes_vector.push(immediate_address);
            binloc += 1 as u16;
            match matched_string {
                "r1" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_add_r1_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_add_r2_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_add_r3_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_add_r4_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        }
        false => {
            // Then it has matched the second register, and we need to get the first register
            let rx =
                Regex::new(r".*?[[:space:]][[:word:]]+.*?[[:space:]]([[:word:]]+).*?").unwrap();
            let other_matched_string = &rx.captures(&elem).unwrap()[1];

            match matched_string {
                "r1" => {
                    match other_matched_string {
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r1_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r1_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r1_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r2" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r2_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r2_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r2_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r3" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r3_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r3_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r3_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r4" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r4_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r4_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_add_r4_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        }
    }
}

pub fn def_mov(elem: String, mut binloc: u16, opcodes_vector: &mut Vec<OperationalCode>) -> u16 {
    // println!("ORIGINAL STRING: {:?}", elem);
    let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
    let rxdigit = Regex::new(r".*[[:space:]]#(-?[[:digit:]]+)").unwrap();
    let matched_string = &rx.captures(&elem).unwrap()[1];
    //let second_matched_string = &rx.captures(&elem).unwrap()[2];
    let immediate_count = &rxdigit.captures_iter(&elem).count();
    let mut use_immediate_value: bool = false;
    let mut rxdigitvalue: i32 = 0;

    match immediate_count {
        1 => {
            rxdigitvalue = rxdigit.captures(&elem).unwrap()[1]
                .parse::<i32>()
                .unwrap_or_default();
            if rxdigitvalue > 255 {
                panic!("Immediate value greater than 255 at: {:?}", elem);
            }
            use_immediate_value = true;
        }
        _ => {
            //println!("immediate_count: {:?} ", immediate_count);
        }
    }

    let another_opcode: OperationalCode = OperationalCode::new(
        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
        crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_inc_pc,
    );
    opcodes_vector.push(another_opcode);
    binloc += 1 as u16;
    let immediate_address: OperationalCode = OperationalCode::new(
        binloc + 1 + crate::bindings::eOpcodes_opcode_Timer_3, rxdigitvalue as u16,
    );
    opcodes_vector.push(immediate_address);
    binloc += 1 as u16;

    match use_immediate_value {
        true => {
            match matched_string {
                "r1" => {

                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_mov_r1_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_mov_r2_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_mov_r3_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_4,
                        crate::bindings::eOpcodes_opcode_mov_r4_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        }
        false => {
            // Then it has matched the second register, and we need to get the first register
            let rx =
                Regex::new(r".*?[[:space:]][[:word:]]+.*?[[:space:]]([[:word:]]+).*?").unwrap();
            let other_matched_string = &rx.captures(&elem).unwrap()[1];

            match matched_string {
                "r1" => {
                    match other_matched_string {
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r1_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r1_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r1_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r2" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r2_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r2_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r2_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r3" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r3_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r3_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r4" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r3_r4,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                "r4" => {
                    match other_matched_string {
                        "r1" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r4_r1,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r2" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r4_r2,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        "r3" => {
                            let another_opcode: OperationalCode = OperationalCode::new(
                                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                                crate::bindings::eOpcodes_opcode_mov_r4_r3,
                            );
                            opcodes_vector.push(another_opcode);
                        }
                        _ => {
                            panic!("Error with statement: {:?}", elem);
                        }
                    }
                    return binloc + 1 as u16;
                }
                _ => {
                    panic!("Error with statement: {:?}", elem);
                }
            }
        }
    }
}

//////////////////////////
pub fn def_shift(
    instruction: i32,
    elem: String,
    binloc: u16,
    opcodes_vector: &mut Vec<OperationalCode>,
) -> u16 {
    match instruction {
        0 => {
            //println!("SHR ELEM: {:?}", elem);
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            match matched_string {
                "r1" => {
                    //println!("SHR R1: {:?}", matched_string);
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_shr_r1,
                    );
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    //println!("SHR R2: {:?}", matched_string);
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_shr_r2,
                    );
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    //println!("SHR R3: {:?}", matched_string);
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_shr_r3,
                    );
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    //println!("SHR R4: {:?}", matched_string);
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_shr_r4,
                    );
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    //println!("SHIFT ERROR");
                    return binloc as u16;
                }
            }
        } // 0 = SHR
        1 => {
            //println!("SHL ELEM: {:?}", elem);
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            match matched_string {
                "r1" => {
                    //println!("SHL R1: {:?}", matched_string);
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_shl_r1,
                    );
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    //println!("SHL R2: {:?}", matched_string);
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_shl_r2,
                    );
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
                "r3" => {
                    //println!("SHL R3: {:?}", matched_string);
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_shl_r3,
                    );
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
                "r4" => {
                    //println!("SHR RL: {:?}", matched_string);
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_shl_r4,
                    );
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
                _ => {
                    //println!("SHIFT ERROR");
                    return binloc as u16;
                }
            }
        } // 1 = SHL
        _ => {
            return binloc;
        }
    }
}

pub fn def_branch(
    instruction: i32,
    elem: String,
    binloc: u16,
    label_list: &mut Vec<Label>,
    unresolved_vector: &mut Vec<Unresolved>,
    opcodes_vector: &mut Vec<OperationalCode>,
) -> u16 {
    match instruction {
        0 => {
            // println!("INSIDE BL: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut() {
                if item.get_name() == matched_string {
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        item.get_location(),
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
            }
            label_list.push(Label::new(matched_string.to_string(), 256 as u16));
            let another_opcode: OperationalCode = OperationalCode::new(
                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                binloc + 1 as u16,
            );
            opcodes_vector.push(another_opcode);
            unresolved_vector.push(Unresolved::new(0, 2, elem.to_string(), binloc));
            return binloc;
        } // BL
        1 => {
            //println!("INSIDE BEQ: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut() {
                //println!("BEQ item_name: {:?}", item.get_name());
                //println!("BEQ regex_match: {:?}", matched_string);
                if item.get_name() == matched_string {
                    //println!("MATCHED LABEL!");
                    let next_opcode: OperationalCode = OperationalCode::new(
                        crate::bindings::eOpcodes_opcode_zero_flag
                            + binloc
                            + crate::bindings::eOpcodes_opcode_Timer_2,
                        item.get_location(),
                    );
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        binloc + 1 as u16,
                    );
                    opcodes_vector.push(next_opcode);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
            }
            label_list.push(Label::new(matched_string.to_string(), 256 as u16));
            let another_opcode: OperationalCode = OperationalCode::new(
                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                binloc + 1 as u16,
            );
            opcodes_vector.push(another_opcode);
            unresolved_vector.push(Unresolved::new(0, 2, elem.to_string(), binloc));
            return binloc;
        } // BEQ zero flag set
        2 => {
            //println!("INSIDE BNE: {:?}", elem);
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut() {
                if item.get_name() == matched_string {
                    //println!("BEQ item_name: {:?}", item.get_name());
                    //println!("BEQ regex_match: {:?}", matched_string);
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        item.get_location(),
                    );
                    let another_opcode: OperationalCode = OperationalCode::new(
                        crate::bindings::eOpcodes_opcode_zero_flag
                            + binloc
                            + crate::bindings::eOpcodes_opcode_Timer_2,
                        binloc + 1 as u16,
                    );
                    opcodes_vector.push(next_opcode);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
            }
            label_list.push(Label::new(matched_string.to_string(), 256 as u16));
            let another_opcode: OperationalCode = OperationalCode::new(
                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                255 + 1 as u16,
            );
            opcodes_vector.push(another_opcode);
            unresolved_vector.push(Unresolved::new(0, 2, elem.to_string(), binloc));
            return binloc;
        } // BNE Zflag not set
        3 => {
            //println!("INSIDE BLT: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut() {
                //println!("BEQ item_name: {:?}", item.get_name());
                //println!("BEQ regex_match: {:?}", matched_string);
                if item.get_name() == matched_string {
                    //println!("MATCHED LABEL!");
                    let next_opcode: OperationalCode = OperationalCode::new(
                        crate::bindings::eOpcodes_opcode_zero_flag
                            + binloc
                            + crate::bindings::eOpcodes_opcode_Timer_2,
                        item.get_location(),
                    );
                    let another_opcode: OperationalCode = OperationalCode::new(
                        crate::bindings::eOpcodes_opcode_carryout_flag
                            + binloc
                            + crate::bindings::eOpcodes_opcode_Timer_2,
                        binloc + 1 as u16,
                    );
                    opcodes_vector.push(next_opcode);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
            }
            label_list.push(Label::new(matched_string.to_string(), 256 as u16));
            let another_opcode: OperationalCode = OperationalCode::new(
                crate::bindings::eOpcodes_opcode_carryout_flag
                    + binloc
                    + crate::bindings::eOpcodes_opcode_Timer_2,
                binloc + 1 as u16,
            );
            opcodes_vector.push(another_opcode);
            unresolved_vector.push(Unresolved::new(0, 2, elem.to_string(), binloc));
            return binloc;
        } // BLT zero flag set and carry flag not
        4 => {
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut() {
                //println!("BEQ item_name: {:?}", item.get_name());
                //println!("BEQ regex_match: {:?}", matched_string);
                if item.get_name() == matched_string {
                    //println!("MATCHED LABEL!");
                    let next_opcode: OperationalCode = OperationalCode::new(
                        crate::bindings::eOpcodes_opcode_carryout_flag
                            + binloc
                            + crate::bindings::eOpcodes_opcode_Timer_2,
                        item.get_location(),
                    );
                    let another_opcode: OperationalCode = OperationalCode::new(
                        crate::bindings::eOpcodes_opcode_zero_flag
                            + binloc
                            + crate::bindings::eOpcodes_opcode_Timer_2,
                        binloc + 1 as u16,
                    );
                    opcodes_vector.push(next_opcode);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
            }
            label_list.push(Label::new(matched_string.to_string(), 256 as u16));
            let another_opcode: OperationalCode = OperationalCode::new(
                crate::bindings::eOpcodes_opcode_zero_flag
                    + binloc
                    + crate::bindings::eOpcodes_opcode_Timer_2,
                binloc + 1 as u16,
            );
            opcodes_vector.push(another_opcode);
            unresolved_vector.push(Unresolved::new(0, 2, elem.to_string(), binloc));
            return binloc;
        } // BGT zero flag not set and carry flag set
        _ => {
            let another_opcode: OperationalCode = OperationalCode::new(
                binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                binloc + 1 as u16,
            );
            opcodes_vector.push(another_opcode);
            return binloc;
        }
    }
}

pub fn def_label(elem: String, binloc: u16, label_list: &mut Vec<Label>) {
    let rx = Regex::new(r"([[:word:]]+):.*?").unwrap(); // strip the : from the end.
    let matched_string = &rx.captures(&elem).unwrap()[1];
    for item in label_list.iter_mut() {
        if item.get_name() == matched_string {
            item.location(binloc);
            return;
        }
    }
    let new_label: Label = Label::new(matched_string.to_string(), binloc);
    label_list.push(new_label);
    return;
}
