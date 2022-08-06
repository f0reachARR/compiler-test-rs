use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Identifier(String),
    Space(usize),
    Equals,
    Character(char),
    GroupBegin,
    GroupEnd,
    RepeatBegin,
    RepeatEnd,
    OptionBegin,
    OptionEnd,
    Separator,
}

#[derive(Debug, PartialEq, Eq)]
struct PositionedToken(Token, usize);

#[derive(Debug)]
struct Tokenizer<'a> {
    input: &'a [char],
    cursor: &'a [char],
}

impl<'a> Tokenizer<'a> {
    fn new<'b>(input: &'b [char]) -> Tokenizer<'b> {
        Tokenizer {
            input,
            cursor: input,
        }
    }

    fn read_character(&mut self) -> Option<PositionedToken> {
        let pos = self.input.len() - self.cursor.len();
        if let ['"', c, '"', rest @ ..] = self.cursor {
            self.cursor = rest;
            Some(PositionedToken(Token::Character(*c), pos))
        } else {
            None
        }
    }
}

fn main() {
    let input = std::env::args().nth(1).expect("no input file");
    let mut file = File::open(input).expect("input file open");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // let input_vec: Vec<char> = &buf.as_str().chars().collect::<Vec<_>>();
    let mut input_buf: &[char] = &buf.as_str().chars().collect::<Vec<_>>();
    let mut tokens = Vec::new();

    while !input_buf.is_empty() {
        let (rest, found) = read_equals(input_buf);
        if found {
            input_buf = rest;
            tokens.push(Token::Equals);
            continue;
        }

        let (rest, c) = read_character(input_buf);
        if let Some(c) = c {
            input_buf = rest;
            tokens.push(Token::Character(c));
            continue;
        }

        let (rest, identifier) = read_identifier(input_buf);
        if let Some(identifier) = identifier {
            input_buf = rest;
            tokens.push(Token::Identifier(identifier));
            continue;
        }

        let (rest, size) = read_space(input_buf);
        if size > 0 {
            input_buf = rest;
            tokens.push(Token::Space(size));
            continue;
        }

        break;
    }

    println!("rest: {}", input_buf.into_iter().collect::<String>());

    dbg!(tokens);
}

fn read_equals(mut input: &[char]) -> (&[char], bool) {
    if let ['=', rest @ ..] = input {
        input = rest;
        (input, true)
    } else if let [':', ':', '=', rest @ ..] = input {
        input = rest;
        (input, true)
    } else {
        (input, false)
    }
}

#[test]
fn read_equals_test() {
    assert_eq!(read_equals(&['"', 'x'][..]), (&['"', 'x'][..], false));
    assert_eq!(read_equals(&['=', 'x'][..]), (&['x'][..], true));
    assert_eq!(read_equals(&[':', ':', '=', 'x'][..]), (&['x'][..], true));
}

fn read_character(mut input: &[char]) -> (&[char], Option<char>) {
    if let ['"', c, '"', rest @ ..] = input {
        input = rest;
        (input, Some(*c))
    } else {
        (input, None)
    }
}

#[test]
fn read_character_test() {
    assert_eq!(
        read_character(&"x".chars().collect::<Vec<_>>()),
        (&['x'][..], None)
    );
    assert_eq!(read_character(&['"', 'x'][..]), (&['"', 'x'][..], None));
    assert_eq!(
        Tokenizer::new(&['"', 'x', '"', 'a'][..]).read_character(),
        Some(PositionedToken(Token::Character('x'), 0))
    );
}

fn read_space(mut input: &[char]) -> (&[char], usize) {
    let mut found = 0;
    while let [' ', rest @ ..] = input {
        input = rest;
        found += 1;
    }
    (input, found)
}

#[test]
fn read_space_test() {
    assert_eq!(read_space(&[' ', ' ', 'x'][..]), (&['x'][..], 2));
    assert_eq!(read_space(&['a', ' ', 'x'][..]), (&['a', ' ', 'x'][..], 0));
    assert_eq!(read_space(&[' ', ' '][..]), (&[][..], 2));
}

fn read_identifier(mut input: &[char]) -> (&[char], Option<String>) {
    let mut identifier = String::new();
    while let [c @ ('0'..='9' | 'a'..='z' | 'A'..='Z'), rest @ ..] = input {
        identifier.push(*c);
        input = rest;
    }
    if identifier.is_empty() {
        (input, None)
    } else {
        (input, Some(identifier))
    }
}

#[test]
fn read_identifier_test() {
    assert_eq!(
        read_identifier(&['a', 'b', 'c', ' ', 'x'][..]),
        (&[' ', 'x'][..], Some(String::from("abc")))
    );
    assert_eq!(
        read_identifier(&['a', 'b', '3', ' ', '$'][..]),
        (&[' ', '$'][..], Some(String::from("ab3")))
    );
    assert_eq!(
        read_identifier(&['$', 'a', 'b', '3', ' '][..]),
        (&['$', 'a', 'b', '3', ' '][..], None)
    );
}
