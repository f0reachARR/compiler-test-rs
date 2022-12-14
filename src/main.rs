use std::{fs::File, io::Read};

mod lexer;
use lexer::Tokenizer;
mod parser;
use parser::Parser;

mod grammer;
mod ll;
mod slr;
fn main() {
    let input = std::env::args().nth(1).expect("no input file");
    let mut file = File::open(input).expect("input file open");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // let input_vec: Vec<char> = &buf.as_str().chars().collect::<Vec<_>>();
    let input_buf: &[char] = &buf.as_str().chars().collect::<Vec<_>>();
    let tokenizer = Tokenizer::new(input_buf);
    if let Some(tokens) = tokenizer.read_definition() {
        // dbg!(&tokens);
        let mut parser = Parser::new(&tokens);
        let rules = parser.eat();

        let gramconv = grammer::Ebnf2Gram::process(rules.unwrap()).unwrap();
        grammer::display_grammer_set(&gramconv.grammer_set);
        dbg!(gramconv.identifier_map);
    }
}
