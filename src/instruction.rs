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

    match use_immediate_value {
        true => {
            // /////////// ===========================================  //////////////////
            //  We have a problem here with the opcodes and need to adjust the C program
            //  that loads the EEProms. To move a constant you need to have:
            //  Timer0/1 load and fetch
            //  Timer 2 load MAR with address of next memory location,
            //  Timer 3 assert MDR on to the databus and latch the register.
            //  I think the code below is wrong. Need to have a think.
            // /////////// ===========================================  //////////////////
            // Then matched string is the register to move immediate into it.
            // println!("immediate = {:?} ", rxdigitvalue);
            match matched_string {
                "r1" => {
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_load_mar,
                    );
                    opcodes_vector.push(another_opcode);
                    binloc += 1 as u16;
                    let immediate_address: OperationalCode = OperationalCode::new(
                        binloc + 1 + crate::bindings::eOpcodes_opcode_Timer_3, rxdigitvalue as u16,
                    );
                    opcodes_vector.push(immediate_address);
                    binloc += 1 as u16;
                    let next_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_3,
                        crate::bindings::eOpcodes_opcode_mov_r1_Const1,
                    );
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
                "r2" => {
                    return binloc + 1 as u16;
                }
                "r3" => {
                    return binloc + 1 as u16;
                }
                "r4" => {
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
            println!("INSIDE BNE: {:?}", elem);
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut() {
                if item.get_name() == matched_string {
                    println!("BEQ item_name: {:?}", item.get_name());
                    println!("BEQ regex_match: {:?}", matched_string);
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
