#![allow(non_snake_case)]
#![allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_must_use)]


pub mod bindings;
pub mod instruction;
pub mod lexer;
pub mod my_operation;
use logos::{Lexer, Logos};
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};
extern crate clap;
use clap::{App, Arg, SubCommand};
extern crate regex;
use bindings::eOpcodes;
use instruction::{def_branch, def_label, def_mov, def_shift};
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
                Token::ADD(inner) => println!("{:?}", inner),
                Token::SUB(inner) => println!("{:?}", inner),
                Token::AND(inner) => println!("{:?}", inner),
                Token::ORR(inner) => println!("{:?}", inner),
                Token::XOR(inner) => println!("{:?}", inner),
                Token::NOT(inner) => println!("{:?}", inner),
                Token::CMP(inner) => println!("{:?}", inner),
                Token::MOV(inner) => {
                    //println!("{:?}", inner);
                    binloc = def_mov(inner, binloc, &mut opcodes_vector);
                }
                Token::LDR(inner) => println!("{:?}", inner),
                Token::STR(inner) => println!("{:?}", inner),
                Token::INC(inner) => println!("{:?}", inner),
                Token::DEC(inner) => println!("{:?}", inner),
                Token::CCF => println!("{:?}", elem),
                Token::MEMORYALIAS(inner) => println!("{:?}", inner),
                Token::PUSH(inner) => println!("{:?}", inner),
                Token::POP(inner) => println!("{:?}", inner),
                Token::LABEL(inner) => {
                    def_label(inner, binloc, &mut label_vector);
                    binloc += 1;
                }
                _ => (),
            }

            match timer_count {
                0 => {
                    // throw away timer 0 & 1 since we don't have any matches and reset the count.
                    //println!("timer_count = 0");
                    opcodes_vector.pop();
                    opcodes_vector.pop();
                    binloc -= 2;
                }
                1 => {
                    //println!("timer_count = 1");
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_2,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                2 => {
                    //println!("timer_count = 2");
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_3,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                3 => {
                    //println!("timer_count = 3");
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_4,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                4 => {
                    //println!("timer_count = 4");
                    let terary_microcode: OperationalCode = OperationalCode::new(
                        binloc + bindings::eOpcodes_opcode_Timer_5,
                        bindings::eOpcodes_opcode_reset_instr_timer,
                    );
                    binloc += 1;
                    opcodes_vector.push(terary_microcode);
                }
                5 => {
                    //println!("timer_count = 5");
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
        "[[:space:]]+add[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    ADD(String),

    #[regex(
        "[[:space:]]+sub[[:space:]]+([[:word:]]+)",
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
        "[[:space:]]+CMP[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    CMP(String),

    #[regex(
        "[[:space:]]+LDR[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    LDR(String),

    #[regex(
        "[[:space:]]+STR[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    STR(String),

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
        "[[:space:]]+pop[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    POP(String),

    #[regex(
        "[[:space:]]+push[[:space:]]+([[:word:]]+)",
        lcaseit,
        priority = 1,
        ignore(ascii_case)
    )]
    PUSH(String),

    #[regex("([[:word:]]+):", lcaseit, ignore(ascii_case))]
    LABEL(String),

    #[regex("=([[:word:]]+)", lcaseit, ignore(ascii_case))]
    MEMORYALIAS(String),

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
