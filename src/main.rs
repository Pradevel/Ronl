mod lexer;

use std::io;
use lexer::lexer::lex;
use std::time::Instant;
use crate::lexer::lexer::types::token::Token;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let start_time = Instant::now();
    let mut tokens: Vec<Token> = lex(input);
    let elapsed_time = start_time.elapsed();
    for token in &tokens {
        match token {
            Token::Letter(c) => println!("LET: {}", c),
            Token::Digit(c) => println!("NUM: {}", c),
            Token::SpecialChar(c) => println!("SCH: {}", c),
            Token::OpenBracket => println!("OB"),
            Token::CloseBracket => println!("CB"),
            Token::Plus => println!("ADD"),
            Token::Minus => println!("MIN"),
            Token::Multiply => println!("MUL"),
            Token::Divide => println!("DIV"),
        }
    }
    println!("Time elapsed: {:?}", elapsed_time);
}
