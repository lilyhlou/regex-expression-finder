#![allow(unused)]
/**
 * thegrep - Tar Heel egrep
 *
 * Author(s): Lily Lou,Taylor Montgomery
 * ONYEN(s): loulh,tayjomo
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */

/**
 * thegrep - Tar Heel egrep
 *
 * This program begins the implement the basics of egrep.
 */
extern crate structopt;

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "thegrep", about = "Tar Heel egrep")]
struct Options {
    /// Show Parsed AST
    #[structopt(short = "p", long = "parse")]
    parse: bool,

    /// Show Tokens
    #[structopt(short = "t", long = "tokens")]
    tokens: bool,

    /// Show NFA
    #[structopt(short = "d", long = "dot")]
    dot: bool,

    ///Generate Acceptable Strings
    #[structopt(short = "g", long = "gen", default_value = "0")]
    outputs: u16,

    ///Regular Expression Pattern
    patterns: String,

    ///Input File
    paths: Vec<String>,
}

pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;
pub mod nfa;
use self::nfa::helpers::nfa_dot;
use self::nfa::NFA;

fn main() {
    let options = Options::from_args();
    eval(&options);
    let result = if options.paths.len() > 0 {
        print_files(&options)
    } else {
        print_stdin(&options)
    };
}

fn eval(options: &Options) {
    if options.tokens {
        eval_show_tokens(options);
    }

    if options.parse {
        eval_show_parse(options);
    }

    if options.dot {
        eval_show_dot(options);
    }

    if options.outputs > 0 {
        eval_gen(options);
    }
}

//if the tokens option is true, it will display the
//command line message in tokenized form
fn eval_show_tokens(options: &Options) {
    let mut tokens = Tokenizer::new(&options.patterns);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}

//if the parse option is true, it will display the
//command line message in parse tree format
fn eval_show_parse(options: &Options) {
    match Parser::parse(Tokenizer::new(&options.patterns)) {
        Ok(statement) => {
            println!("{:?}", statement);
            //parse method is returning a statement
        }
        Err(msg) => eprintln!("thegrep: {}", msg),
    }
    print!("\n");
}
//if dot option is true, it will produce a dot
//representation of thegrep
fn eval_show_dot(options: &Options) {
    let nfa = NFA::from(&options.patterns).unwrap();
    println!("{}", nfa_dot(&nfa));
    std::process::exit(0);
}

fn eval_gen(options: &Options) {
    let nfa = NFA::from(&format!("{}", &options.patterns)).unwrap();

    for i in 0..options.outputs {
        let string = nfa.gen();
        println!("{}", string);
    }
    std::process::exit(0);
}

fn print_stdin(options: &Options) -> io::Result<()> {
    let nfa = NFA::from(&format!(".*{}.*", &options.patterns)).unwrap();
    let stdin = io::stdin();
    let reader = stdin.lock();
    print_lines(reader, &nfa)
}

use std::fs::File;
use std::io;
use std::io::BufRead;

fn print_files(options: &Options) -> io::Result<()> {
    let nfa = NFA::from(&format!(".*{}.*", &options.patterns)).unwrap();
    for path in options.paths.iter() {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        print_lines(reader, &nfa)?;
    }

    Ok(())
}

fn print_lines<R: BufRead>(reader: R, nfa: &NFA) -> io::Result<()> {
    for line_result in reader.lines() {
        let line_result = line_result?;
        if nfa.accepts(&line_result) {
            println!("{}", &line_result);
        }
    }
    Ok(())
}
