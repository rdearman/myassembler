#![allow(non_snake_case)]
#![allow(unused_imports)]

mod bindings;
mod lexer;
use std::fs::File;
use std::io::Write;   
use std::io::{BufRead, BufReader, BufWriter};
use logos::{Logos, Lexer};
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main()
{
    let matches = App::new("myassembler")
                    .version("0.1.0")
                    .author("Rick Dearman <rick@rdearman.org>")
                    .about("Generates binary file for my 8-bit breadboard computer")
                    // .arg(Arg::with_name("IN")
                    //     .help("Sets the input file to use")
                    //     .index(1))
                    .arg(Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("INFILE")
                        .help("Sets the input file name")
                        .takes_value(true))
                    .arg(Arg::with_name("output")
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
    let mut opcodes_vector: Vec::<u8> = vec![];

    opcodes_vector.push( bindings::eOpcodes_opcode_nop);
    opcodes_vector.push( bindings::eOpcodes_opcode_mov_r2_r4);
    opcodes_vector.push( bindings::eOpcodes_opcode_mov_sp_r4);
    opcodes_vector.push( bindings::eOpcodes_opcode_mov_mr_r4);

    let opcode_buffer: &[u8] = &opcodes_vector;

    bfile.write_all(opcode_buffer).unwrap();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines()
    {
        let tline = &line.unwrap();
        // println!("{:?}", tline);
        let lex = Token::lexer( tline );
        for elem in lex
        {
            match elem
            {
                Token::BL(_)  => println!("{:?}", elem),
                Token::BEQ(_) => println!("{:?}", elem),
                Token::BLT(_) => println!("{:?}", elem),
                Token::BGT(_) => println!("{:?}", elem),
                Token::ADD(_)  => println!("{:?}", elem),
                Token::SUB(_) => println!("{:?}", elem),
                Token::AND(_) => println!("{:?}", elem),
                Token::ORR(_) => println!("{:?}", elem),
                Token::XOR(_)  => println!("{:?}", elem),
                Token::NOT(_) => println!("{:?}", elem),
                Token::CMP(_) => println!("{:?}", elem),
                Token::MOV(_) => println!("{:?}", elem),
                Token::LDR(_)  => println!("{:?}", elem),
                Token::STR(_) => println!("{:?}", elem),
                Token::SHR(_) => println!("{:?}", elem),
                Token::SHL(_) => println!("{:?}", elem),
                Token::INC(_)  => println!("{:?}", elem),
                Token::DEC(_) => println!("{:?}", elem),
                Token::CCF => println!("{:?}", elem),
                Token::MEMORYALIAS(_) => println!("{:?}", elem),
                Token::PUSH(_) => println!("{:?}", elem),
                Token::POP(_)  => println!("{:?}", elem),
                //Token::COMMENT  => (),
                _  => (),
            }
        }
    }
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

    #[regex("(=[[:word:]]+)",lcaseit,  ignore(ascii_case) )]
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
    
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn lcaseit(lex: &mut Lexer<Token>) -> String
{
    let slice = lex.slice();
    let my_string:String = slice[..slice.len()].to_string();
    return my_string.to_lowercase();
}

