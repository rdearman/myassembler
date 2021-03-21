#![allow(non_snake_case)]
#![allow(unused_imports)]
#[allow(dead_code)]

mod bindings;
mod lexer;
mod instruction;
use std::fs::File;
use std::io::Write;   
use std::io::{BufRead, BufReader, BufWriter};
use logos::{Logos, Lexer};
extern crate clap;
use clap::{Arg, App, SubCommand};
extern crate regex;
use regex::Regex;
use instruction::{Label, OperationalCode};
use bindings::eOpcodes;
extern crate byteorder;
use byteorder::{WriteBytesExt, LittleEndian};

fn main()
{
    let matches = App::new("myassembler")
                    .version("0.1.0")
                    .author("Rick Dearman <rick@rdearman.org>")
                    .about("Generates binary file for my 8-bit breadboard computer")
                    //.setting(AppSettings::ArgRequiredElseHelp)
                    .arg(Arg::with_name("input")
                        .required(true)
                        .short("i")
                        .long("input")
                        .value_name("INFILE")
                        .help("Sets the input file name")
                        .takes_value(true))
                    .arg(Arg::with_name("output")
                        .required(true)
                        .short("o")
                        .long("output")
                        .value_name("OUTFILE")
                        .default_value("output.bin")
                        .help("Sets the output file name (defaults to 'output.bin'")
                        .takes_value(true))
                    .get_matches();

    let filename = matches.value_of("input").unwrap();
    //let filename = matches.value_of("input").unwrap_or(matches.value_of("IN").unwrap());
    let mut bfile = BufWriter::new(File::create(matches.value_of("output").unwrap()).unwrap());
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut opcodes_vector: Vec::<OperationalCode> = vec![];
    let mut label_vector = vec![];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut binloc =0; 
    for line in reader.lines()
    {
        // First thing push the two instructions which ALWAYS come first.
        let primary_microcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_0, bindings::eOpcodes_opcode_fetch_instruction);
        binloc += 1;
        let secondary_microcode: OperationalCode = OperationalCode::new( binloc  + bindings::eOpcodes_opcode_Timer_1, bindings::eOpcodes_opcode_load_instruction);
        binloc += 1;
        opcodes_vector.push(primary_microcode);
        opcodes_vector.push(secondary_microcode);
        let timer_count = binloc;
        let tline = &line.unwrap();
        let lex = Token::lexer( tline );
        for elem in lex
        {
             match elem
            {
                Token::BL(inner)  =>
                {
                    binloc = def_branch(0, inner, binloc, &mut label_vector, &mut opcodes_vector) ;
                },
                Token::BEQ(inner) =>
                {
                    binloc = def_branch(1, inner, binloc, &mut label_vector, &mut opcodes_vector) ;
                },
                Token::BNE(inner)  =>
                {
                    binloc = def_branch(2, inner, binloc, &mut label_vector, &mut opcodes_vector) ;
                },
                 Token::BLT(inner) =>
                {
                    binloc = def_branch(3, inner, binloc, &mut label_vector, &mut opcodes_vector) ;
                },
                Token::BGT(inner) =>
                {
                    binloc = def_branch(4, inner, binloc, &mut label_vector, &mut opcodes_vector) ;
                },
                Token::SHR(inner) =>                 {
                    binloc = def_shift(0, inner, binloc, &mut opcodes_vector) ;
                },
                Token::SHL(inner) =>                 {
                    binloc = def_shift(1, inner, binloc, &mut opcodes_vector) ;
                },
                Token::ADD(inner)  => println!("{:?}", inner),
                Token::SUB(inner) => println!("{:?}", inner),
                Token::AND(inner) => println!("{:?}", inner),
                Token::ORR(inner) => println!("{:?}", inner),
                Token::XOR(inner)  => println!("{:?}", inner),
                Token::NOT(inner) => println!("{:?}", inner),
                Token::CMP(inner) => println!("{:?}", inner),
                Token::MOV(inner) => println!("{:?}", inner),
                Token::LDR(inner)  => println!("{:?}", inner),
                Token::STR(inner) => println!("{:?}", inner),
                Token::INC(inner)  => println!("{:?}", inner),
                Token::DEC(inner) => println!("{:?}", inner),
                Token::CCF => println!("{:?}", elem),
                Token::MEMORYALIAS(inner) => println!("{:?}", inner),
                Token::PUSH(inner) => println!("{:?}", inner),
                Token::POP(inner)  => println!("{:?}", inner),
                Token::LABEL(inner)  => { 
                    label_vector.push(def_label(inner, binloc));
                    binloc += 1;
                    },
                _  => (),
            }
        }
        match timer_count
        {
            1 => {
                let terary_microcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_reset_instr_timer);
                binloc += 1;
                opcodes_vector.push(terary_microcode);
            },
            2 => {
                let terary_microcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_3, bindings::eOpcodes_opcode_reset_instr_timer);
                binloc += 1;
                opcodes_vector.push(terary_microcode);
            },
            3 => {
                let terary_microcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_4, bindings::eOpcodes_opcode_reset_instr_timer);
                binloc += 1;
                opcodes_vector.push(terary_microcode);
            },
            4 => {
                let terary_microcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_5, bindings::eOpcodes_opcode_reset_instr_timer);
                binloc += 1;
                opcodes_vector.push(terary_microcode);
            },
            5 => {
                let terary_microcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_6, bindings::eOpcodes_opcode_reset_instr_timer);
                binloc += 1;
                opcodes_vector.push(terary_microcode);
            },
            _ => {
                let terary_microcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_7, bindings::eOpcodes_opcode_reset_instr_timer);
                binloc += 1;
                opcodes_vector.push(terary_microcode);
            },
        }

    }

for i in opcodes_vector.iter_mut()
{
    bfile.write_u16::<LittleEndian>(i.get_memory_location());
}
    /*   Block for writing out buffer (old code)
    let opcode_buffer: &[u16] = &opcodes_vector.memory_location;
    bfile.write_all(opcode_buffer).unwrap();
*/

}

fn def_shift( instruction: i32, elem: String, binloc: u16, opcodes_vector: &mut Vec::<OperationalCode>) -> u16
{
    match instruction
    {
        0 => {
            println!("SHR ELEM: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            match matched_string
            {
                "r1" => {
                    println!("SHR R1: {:?}", matched_string );
                    let another_opcode: OperationalCode = OperationalCode::new( binloc  + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_shr_r1);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                },
                "r2" => {
                    println!("SHR R2: {:?}", matched_string );
                    let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_shr_r2);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                },
                "r3" => {
                    println!("SHR R3: {:?}", matched_string );
                    let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_shr_r3);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                },
                "r4" => {
                    println!("SHR R4: {:?}", matched_string );
                    let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_shr_r4);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                },
                _ => { println!("SHIFT ERROR"); return binloc as u16; },
            }

        }, // 0 = SHR
        1 => { 
            println!("SHL ELEM: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            match matched_string
            {
                "r1" => {
                    println!("SHL R1: {:?}", matched_string );
                    let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_shl_r1);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                },
                "r2" => {
                    println!("SHL R2: {:?}", matched_string );
                    let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_shl_r2);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                },
                "r3" => {
                    println!("SHL R3: {:?}", matched_string );
                    let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_shl_r3);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                },
                "r4" => {
                    println!("SHR RL: {:?}", matched_string );
                    let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, bindings::eOpcodes_opcode_shl_r4);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                },
                _ => { println!("SHIFT ERROR"); return binloc as u16; },
            }
        }, // 1 = SHL
        _ => {
            return binloc;
        },
    }
}

fn def_branch(instruction: i32, elem: String, binloc: u16, label_list: &mut Vec<Label>, opcodes_vector: &mut Vec::<OperationalCode>) -> u16
{
    match instruction
    {
        0 => {
            // println!("INSIDE BL: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut()
            {
                // println!("item_name: {:?}", item.get_name());
                // println!("regex_match: {:?}", matched_string);
                if item.get_name() == matched_string
                {
                    // println!("MATCHED LABEL!");
                    let next_opcode: OperationalCode = OperationalCode::new(binloc + bindings::eOpcodes_opcode_Timer_2, item.get_location());
                    opcodes_vector.push(next_opcode);
                    return binloc + 1 as u16;
                }
            }
            let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
            opcodes_vector.push(another_opcode);
            return binloc;
        }, // BL
        1 => {
            //println!("INSIDE BEQ: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut()
            {
                //println!("BEQ item_name: {:?}", item.get_name());
                //println!("BEQ regex_match: {:?}", matched_string);
                if item.get_name() == matched_string
                {
                    //println!("MATCHED LABEL!");
                    let next_opcode: OperationalCode = OperationalCode::new(bindings::eOpcodes_opcode_zero_flag + binloc + bindings::eOpcodes_opcode_Timer_2, item.get_location());
                    let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
                    opcodes_vector.push(next_opcode);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
            }
            let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1  as u16);
            opcodes_vector.push(another_opcode);
            return binloc ;
        }, // BEQ zero flag set
        2 => {
            //println!("INSIDE BEQ: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut()
            {
                //println!("BEQ item_name: {:?}", item.get_name());
                //println!("BEQ regex_match: {:?}", matched_string);
                if item.get_name() == matched_string
                {
                    //println!("MATCHED LABEL!");
                    let next_opcode: OperationalCode = OperationalCode::new(binloc + bindings::eOpcodes_opcode_Timer_2, item.get_location());
                    let another_opcode: OperationalCode = OperationalCode::new( bindings::eOpcodes_opcode_zero_flag + binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
                    opcodes_vector.push(next_opcode);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
            }
            let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
            opcodes_vector.push(another_opcode);
            return binloc ;
        }, // BNE Zflag not set
        3 => {
            //println!("INSIDE BLT: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut()
            {
                //println!("BEQ item_name: {:?}", item.get_name());
                //println!("BEQ regex_match: {:?}", matched_string);
                if item.get_name() == matched_string
                {
                    //println!("MATCHED LABEL!");
                    let next_opcode: OperationalCode = OperationalCode::new(bindings::eOpcodes_opcode_zero_flag + binloc + bindings::eOpcodes_opcode_Timer_2, item.get_location());
                    let another_opcode: OperationalCode = OperationalCode::new( bindings::eOpcodes_opcode_carryout_flag + binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
                    opcodes_vector.push(next_opcode);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
            }
            let another_opcode: OperationalCode = OperationalCode::new( bindings::eOpcodes_opcode_carryout_flag + binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
            opcodes_vector.push(another_opcode);
            return binloc ;
        }, // BLT zero flag set and carry flag not
        4 => {
            //println!("INSIDE BLT: {:?}", elem );
            let rx = Regex::new(r".*[[:space:]]([[:word:]]+)").unwrap();
            let matched_string = &rx.captures(&elem).unwrap()[1];
            for item in label_list.iter_mut()
            {
                //println!("BEQ item_name: {:?}", item.get_name());
                //println!("BEQ regex_match: {:?}", matched_string);
                if item.get_name() == matched_string
                {
                    //println!("MATCHED LABEL!");
                    let next_opcode: OperationalCode = OperationalCode::new( bindings::eOpcodes_opcode_carryout_flag + binloc + bindings::eOpcodes_opcode_Timer_2, item.get_location());
                    let another_opcode: OperationalCode = OperationalCode::new( bindings::eOpcodes_opcode_zero_flag + binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
                    opcodes_vector.push(next_opcode);
                    opcodes_vector.push(another_opcode);
                    return binloc + 1 as u16;
                }
            }
            let another_opcode: OperationalCode = OperationalCode::new( bindings::eOpcodes_opcode_zero_flag + binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
            opcodes_vector.push(another_opcode);
            return binloc ;
        }, // BGT zero flag not set and carry flag set
        _ => {
            let another_opcode: OperationalCode = OperationalCode::new( binloc + bindings::eOpcodes_opcode_Timer_2, binloc + 1 as u16);
            opcodes_vector.push(another_opcode);
            return binloc;
        },
    }

}

fn def_label(elem: String, binloc: u16 ) -> Label
{
    let rx = Regex::new(r"([[:word:]]+):.*?").unwrap(); // strip the : from the end.
    let matched_string = &rx.captures(&elem).unwrap()[1];
    let new_label: Label = Label::new(matched_string.to_string(), binloc);
    return new_label;
}

fn lcaseit(lex: &mut Lexer<Token>) -> String
{
    let slice = lex.slice();
    let my_string:String = slice[..slice.len()].to_string();
    return my_string.to_lowercase();
}


#[derive(Logos, Debug, PartialEq)]
enum Token {

    #[regex("[[:space:]]+mov[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    MOV(String),

    #[regex("[[:space:]]+add[[:space:]]+([[:word:]]+)",  lcaseit, priority = 1, ignore(ascii_case))]
    ADD(String),

    #[regex("[[:space:]]+sub[[:space:]]+([[:word:]]+)",  lcaseit, priority = 1,ignore(ascii_case))]
    SUB(String),

    #[regex("[[:space:]]+and[[:space:]]+([[:word:]]+)",  lcaseit,priority = 1, ignore(ascii_case))]
    AND(String),

    #[regex("[[:space:]]+ORR[[:space:]]+([[:word:]]+)",  lcaseit, priority = 1,ignore(ascii_case))]
    ORR(String),

    #[regex("[[:space:]]+XOR[[:space:]]+([[:word:]]+)",  lcaseit,priority = 1, ignore(ascii_case))]
    XOR(String),

    #[regex("[[:space:]]+not[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    NOT(String),

    #[regex("[[:space:]]+CMP[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    CMP(String),

    #[regex("[[:space:]]+LDR[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    LDR(String),

    #[regex("[[:space:]]+STR[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    STR(String),

    #[regex("[[:space:]]+SHR[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    SHR(String),

    #[regex("[[:space:]]+SHL[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    SHL(String),

    #[regex("[[:space:]]+INC[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    INC(String),

    #[regex("[[:space:]]+DEC[[:space:]]+([[:word:]]+)", lcaseit,priority = 1, ignore(ascii_case))]
    DEC(String),

    #[regex("[[:space:]]+CCF[[:space:]]+", ignore(ascii_case))]
    CCF,

    #[regex("[[:space:]]+bl[[:space:]]+([[:word:]]+)",lcaseit, priority = 1, ignore(ascii_case))]
    BL(String),

    #[regex("[[:space:]]+beq[[:space:]]+([[:word:]]+)", lcaseit,priority = 1, ignore(ascii_case))]
    BEQ(String),

    #[regex("[[:space:]]+bne[[:space:]]+([[:word:]]+)", lcaseit,priority = 1, ignore(ascii_case))]
    BNE(String),

    #[regex("[[:space:]]+blt[[:space:]]+([[:word:]]+)",lcaseit, priority = 1, ignore(ascii_case))]
    BLT(String),

    #[regex("[[:space:]]+bgt[[:space:]]+([[:word:]]+)", lcaseit,priority = 1, ignore(ascii_case))]
    BGT(String),

    #[regex("[[:space:]]+pop[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    POP(String),

    #[regex("[[:space:]]+push[[:space:]]+([[:word:]]+)", lcaseit, priority = 1, ignore(ascii_case))]
    PUSH(String),

    #[regex("([[:word:]]+):",lcaseit,  ignore(ascii_case) )]
    LABEL(String),

    #[regex("=([[:word:]]+)",lcaseit,  ignore(ascii_case) )]
    MEMORYALIAS(String),

    #[token("r1", ignore(ascii_case))]
    R1,

    #[token("r2", ignore(ascii_case))]
    R2,

    #[token("r3", ignore(ascii_case))]
    R3,

    #[token("r4", ignore(ascii_case))]
    R4,

    #[token("ir", ignore(ascii_case))]
    IR,

    #[token("PC", ignore(ascii_case))]
    PC,

    #[token(".")]
    Period,

    #[token(",")]
    COMMA,

    #[token("[")]
    SquareBracketOpen,

    #[token("]")]
    SquareBracketClose,

    #[token("{")]
    CurlyBracketOpen,

    #[token("}")]
    CurlyBracketClose,

    #[regex("#([[:digit:]]+)")]
    IMMEDIATE,

    // Or regular expressions.
    #[regex("[[:word:]]+", priority = 2)]
    JMPLOC,

    // Tokens can be literal strings, of any length.
    #[regex("[;]+.*", logos::skip)]
    #[regex("[[/]{2}.*]", logos::skip)]
    COMMENT,

    #[regex(r"(?s)/\*.*\*/", logos::skip)]
    MULTILINECOMMENT,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}


