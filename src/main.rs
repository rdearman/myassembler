#![allow(non_snake_case)]
#![allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_must_use)]
pub mod bindings;
pub mod instruction;
pub mod my_operation;
use logos::{Lexer, Logos};
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};
extern crate clap;
use clap::{App, Arg, SubCommand};
extern crate regex;
use bindings::eOpcodes;
use instruction::*;
// use instruction::{def_branch, def_label, def_mov, def_shift, def_add, def_sub, def_logic, def_inc_dec, def_load_memory,def_store_memory};
use my_operation::{Label, OperationalCode, Unresolved};
use regex::Regex;
extern crate byteorder;
use byteorder::{LittleEndian, WriteBytesExt};

fn main() {
    let matches = App::new("myassembler")
        .version("0.1.0")
        .author("Rick Dearman <rick@rdearman.org>")
        .about("Generates binary file for my 8-bit breadboard computer")
        //.setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("input")
                .required(true)
                .short("i")
                .long("input")
                .value_name("INFILE")
                .help("Sets the input file name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .required(true)
                .short("o")
                .long("output")
                .value_name("OUTFILE")
                .default_value("output.bin")
                .help("Sets the output file name (defaults to 'output.bin'")
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("input").unwrap();
    let mut bfile = BufWriter::new(File::create(matches.value_of("output").unwrap()).unwrap());
    let file = File::open(filename).unwrap();
    let first_pass = BufReader::new(file);
    let mut opcodes_vector: Vec<OperationalCode> = vec![];
    let mut label_vector = vec![];
    let mut unresolved_vector: Vec<Unresolved> = vec![];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut binloc = 0;
    for line in first_pass.lines() {
        let tline = &line.unwrap();
        let lex2 = Token::lexer(tline);
        for elem in lex2 {
            let timer_count = binloc;
            // First thing push the two instructions which ALWAYS come first.
            let primary_microcode: OperationalCode = OperationalCode::new(
                binloc + bindings::eOpcodes_opcode_Timer_0,
                bindings::eOpcodes_opcode_fetch_instruction,
            );
            binloc += 1;
            let secondary_microcode: OperationalCode = OperationalCode::new(
                binloc + bindings::eOpcodes_opcode_Timer_1,
                bindings::eOpcodes_opcode_load_instruction,
            );
            binloc += 1;
            opcodes_vector.push(primary_microcode);
            opcodes_vector.push(secondary_microcode);

            match elem {
                Token::BL(inner) => {
                    binloc = def_branch(
                        0,
                        inner,
                        binloc,
                        &mut label_vector,
                        &mut unresolved_vector,
                        &mut opcodes_vector,
                    );
                }
                Token::BEQ(inner) => {
                    binloc = def_branch(
                        1,
                        inner,
                        binloc,
                        &mut label_vector,
                        &mut unresolved_vector,
                        &mut opcodes_vector,
                    );
                }
                Token::BNE(inner) => {
                    binloc = def_branch(
                        2,
                        inner,
                        binloc,
                        &mut label_vector,
                        &mut unresolved_vector,
                        &mut opcodes_vector,
                    );
                }
                Token::BLT(inner) => {
                    binloc = def_branch(
                        3,
                        inner,
                        binloc,
                        &mut label_vector,
                        &mut unresolved_vector,
                        &mut opcodes_vector,
                    );
                }
                Token::BGT(inner) => {
                    binloc = def_branch(
                        4,
                        inner,
                        binloc,
                        &mut label_vector,
                        &mut unresolved_vector,
                        &mut opcodes_vector,
                    );
                }
                Token::SHR(inner) => {
                    binloc = def_shift(0, inner, binloc, &mut opcodes_vector);
                }
                Token::SHL(inner) => {
                    binloc = def_shift(1, inner, binloc, &mut opcodes_vector);
                }
                Token::ADD(inner) => {
                    binloc = def_add(inner, binloc, &mut opcodes_vector);
                },
                Token::SUB(inner) => {
                    binloc = def_sub(inner, binloc, &mut opcodes_vector);
                },
                Token::AND(inner) => {
                    binloc = def_logic(2, inner, binloc, &mut opcodes_vector);
                },
                Token::ORR(inner) => {
                    binloc = def_logic(3, inner, binloc, &mut opcodes_vector);
                },
                Token::XOR(inner) => {
                    binloc = def_logic(1, inner, binloc, &mut opcodes_vector);
                },
                Token::NOT(inner) => {
                    binloc = def_logic(0, inner, binloc, &mut opcodes_vector);
                },
                Token::CMP(inner) => println!("Comparisions require hardware update with a 74LS684 chip {:?}", inner),
                Token::MOV(inner) => {
                    binloc = def_mov(inner, binloc, &mut opcodes_vector);
                }
                Token::LDRV(inner) => {
                    binloc = def_load_memory(0, inner, binloc, &mut opcodes_vector);
                }
                Token::LDRM(inner) => {
                    binloc = def_load_memory(1, inner, binloc, &mut opcodes_vector);
                }
                Token::STRV(inner) => {
                    binloc = def_store_memory(0, inner, binloc, &mut opcodes_vector);
                }
                Token::STRM(inner) => {
                    binloc = def_store_memory(1, inner, binloc, &mut opcodes_vector);
                }
                Token::INC(inner) => {
                    binloc = def_inc_dec(0, inner, binloc, &mut opcodes_vector);
                },
                Token::DEC(inner) => {
                    binloc = def_inc_dec(1, inner, binloc, &mut opcodes_vector);
                },
                Token::CCF => {
                    let another_opcode: OperationalCode = OperationalCode::new(
                        binloc + crate::bindings::eOpcodes_opcode_Timer_2,
                        crate::bindings::eOpcodes_opcode_load_mar + crate::bindings::eOpcodes_opcode_clear_carry,
                    );
                    opcodes_vector.push(another_opcode);
                    binloc += 1 as u16;
                },
                Token::PUSH(inner) => {
                    binloc = def_push( inner, binloc, &mut opcodes_vector);
                }
                Token::POP(inner) => {
                    binloc = def_pop( inner, binloc, &mut opcodes_vector);
                }
                Token::LABEL(inner) => {
                    def_label(inner, binloc, &mut label_vector);
                    binloc += 1;
                }
                _ => (),
            }

        /*  Match the number of bytecode instructions and sent the reset signal
            to avoid wasting clock cyles.
        */
            match timer_count {
                0 => {
                    opcodes_vector.pop();
                    opcodes_vector.pop();
                    binloc -= 2;
                }
                1 => {
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_2,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                2 => {
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_3,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                3 => {
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_4,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                4 => {
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_5,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                5 => {
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_6,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                _ => {}
            }
        }
    }

    let mut dummy_vector = vec![];

    for parse_problem in unresolved_vector.iter_mut() {
        // go through unresoved issues and repair (now that you've passed through the entire file)
        if parse_problem.get_resolution_type() == 0
        // Label problem
        {
            def_branch(
                parse_problem.get_arg_integer(),
                parse_problem.get_resolution_string(),
                parse_problem.get_memory_location(),
                &mut label_vector,
                &mut dummy_vector,
                &mut opcodes_vector,
            );
        }
    }

    for x in 0..0xffff {
        for i in opcodes_vector.iter_mut() {
            if i.get_memory_location() == x {
                let _unused = bfile.write_u16::<LittleEndian>(i.get_memory_location());
            } else {
                let _unused = bfile.write_u16::<LittleEndian>(0);
            }
        }
    }
}

pub fn lcaseit(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    let my_string: String = slice[..slice.len()].to_string();
    return my_string.to_lowercase();
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Tokens can be literal strings, of any length.
    #[regex("[;]+.*", logos::skip, priority = 1)]
    #[regex("[[/]{2}].*", logos::skip, priority = 2)]
    COMMENT,

    #[regex(r"(?s)/\*.*\*/", logos::skip, priority = 1)]
    MULTILINECOMMENT,

    #[regex(
        "[[:space:]]+MOV[[:space:]]+([[:word:]])+[[:space:]]+#*(-?[[:word:]])+",
        lcaseit,
        priority = 2,
        ignore(ascii_case)
    )]
    MOV(String),

    #[regex(
        "[[:space:]]+ADD[[:space:]]+([[:word:]])+[[:space:]]+#*(-?[[:word:]])+",
        lcaseit,
        priority = 2,
        ignore(ascii_case)
    )]
    ADD(String),

    #[regex(
        "[[:space:]]+ADD[[:space:]]+([[:word:]])+[[:space:]]+#*(-?[[:word:]])+",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    SUB(String),

    #[regex(
        "[[:space:]]+and[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    AND(String),

    #[regex(
        "[[:space:]]+ORR[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    ORR(String),

    #[regex(
        "[[:space:]]+XOR[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    XOR(String),

    #[regex(
        "[[:space:]]+not[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    NOT(String),

    #[regex(
        "[[:space:]]+ADD[[:space:]]+([[:word:]])+[[:space:]]+#*(-?[[:word:]])+",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    CMP(String),

    #[regex(
        r"[[:space:]]+ldr[[:space:]]+([[:word:]])+[[:space:]]+\$*(-?[[:word:]])+",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    LDRV(String),

    #[regex(
        r"[[:space:]]+ldr[[:space:]]+([[:word:]]+[[:space:]]+[\[]+.*)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    LDRM(String),

    #[regex(
        r"[[:space:]]+str[[:space:]]+([[:word:]])+[[:space:]]+\$*(-?[[:word:]])+",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    STRV(String),

    #[regex(
        r"[[:space:]]+str[[:space:]]+([[:word:]]+[[:space:]]+[\[]+.*)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    STRM(String),
    // #[regex(
    //     "[[:space:]]+ADD[[:space:]]+([[:word:]])+[[:space:]]+#*(-?[[:word:]])+",
    //     lcaseit,
    //     priority = 1,
    //     ignore(ascii_case)
    // )]
    // STR(String),

    #[regex(
        "[[:space:]]+SHR[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    SHR(String),

    #[regex(
        "[[:space:]]+SHL[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    SHL(String),

    #[regex(
        "[[:space:]]+INC[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    INC(String),

    #[regex(
        "[[:space:]]+DEC[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    DEC(String),

    #[regex("[[:space:]]+CCF[[:space:]]+", ignore(ascii_case))]
    CCF,

    #[regex(
        "[[:space:]]+bl[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    BL(String),

    #[regex(
        "[[:space:]]+beq[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    BEQ(String),

    #[regex(
        "[[:space:]]+bne[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    BNE(String),

    #[regex(
        "[[:space:]]+blt[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    BLT(String),

    #[regex(
        "[[:space:]]+bgt[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    BGT(String),

    #[regex(
        "[[:space:]]+pop(.*)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    POP(String),

    #[regex(
        "[[:space:]]+push(.*)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    PUSH(String),

    #[regex("([[:word:]]+):", lcaseit, ignore(ascii_case))]
    LABEL(String),


    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
