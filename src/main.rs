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
    TokenEnd,
    LineEnd,
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

    fn get_pos(&self) -> usize {
        self.input.len() - self.cursor.len()
    }

    fn read_definition(mut self) -> Vec<PositionedToken> {
        let mut tokens = Vec::new();
        loop {
            if let Some(t) = self.read_character() {
                tokens.push(t);
            } else if let Some(t) = self.read_identifier() {
                tokens.push(t);
            } else if let Some(t) = self.read_equals() {
                tokens.push(t);
            } else if let Some(t) = self.read_single_token() {
                tokens.push(t);
            } else if let Some(t) = self.read_space() {
                tokens.push(t);
            } else if let Some(t) = self.read_lineend() {
                tokens.push(t);
            } else {
                break;
            }
        }
        tokens
    }

    fn read_character(&mut self) -> Option<PositionedToken> {
        let pos = self.get_pos();
        if let ['"', c, '"', rest @ ..] = self.cursor {
            self.cursor = rest;
            Some(PositionedToken(Token::Character(*c), pos))
        } else {
            None
        }
    }

    fn read_space(&mut self) -> Option<PositionedToken> {
        let pos = self.get_pos();
        // let spaces = self.cursor.iter().take_while(|x| x == ' ').count();
        let mut found = 0;
        while let [' ', rest @ ..] = self.cursor {
            self.cursor = rest;
            found += 1;
        }

        if found > 0 {
            Some(PositionedToken(Token::Space(found), pos))
        } else {
            None
        }
    }

    fn read_lineend(&mut self) -> Option<PositionedToken> {
        let pos = self.get_pos();
        // let spaces = self.cursor.iter().take_while(|x| x == ' ').count();
        let mut found = 0;
        while let [_c @ ('\n' | '\r'), rest @ ..] = self.cursor {
            self.cursor = rest;
            found += 1;
        }

        if found > 0 {
            Some(PositionedToken(Token::LineEnd, pos))
        } else {
            None
        }
    }

    fn read_equals(&mut self) -> Option<PositionedToken> {
        let pos = self.get_pos();
        if let ['=', rest @ ..] = self.cursor {
            self.cursor = rest;
            Some(PositionedToken(Token::Equals, pos))
        } else if let [':', ':', '=', rest @ ..] = self.cursor {
            self.cursor = rest;
            Some(PositionedToken(Token::Equals, pos))
        } else {
            None
        }
    }

    fn read_identifier(&mut self) -> Option<PositionedToken> {
        let pos = self.get_pos();
        let mut identifier = String::new();
        while let [c @ ('0'..='9' | 'a'..='z' | 'A'..='Z'), rest @ ..] = self.cursor {
            identifier.push(*c);
            self.cursor = rest;
        }
        if identifier.is_empty() {
            None
        } else {
            Some(PositionedToken(Token::Identifier(identifier), pos))
        }
    }

    fn read_single_token(&mut self) -> Option<PositionedToken> {
        let pos = self.get_pos();
        match self.cursor {
            ['[', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::OptionBegin, pos))
            }
            [']', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::OptionEnd, pos))
            }
            ['(', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::GroupBegin, pos))
            }
            [')', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::GroupEnd, pos))
            }
            ['{', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::RepeatBegin, pos))
            }
            ['}', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::RepeatEnd, pos))
            }
            [',', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::Separator, pos))
            }
            [';', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::TokenEnd, pos))
            }
            _ => None,
        }
    }
}

fn main() {
    let input = std::env::args().nth(1).expect("no input file");
    let mut file = File::open(input).expect("input file open");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // let input_vec: Vec<char> = &buf.as_str().chars().collect::<Vec<_>>();
    let input_buf: &[char] = &buf.as_str().chars().collect::<Vec<_>>();
    let tokenizer = Tokenizer::new(input_buf);
    let tokens = tokenizer.read_definition();

    dbg!(tokens);
}

#[test]
fn read_equals_test() {
    assert_eq!(Tokenizer::new(&['a'][..]).read_equals(), None);
    assert_eq!(
        Tokenizer::new(&['=', 'x'][..]).read_equals(),
        Some(PositionedToken(Token::Equals, 0))
    );
    assert_eq!(
        Tokenizer::new(&[':', ':', '=', 'x'][..]).read_equals(),
        Some(PositionedToken(Token::Equals, 0))
    );
}

#[test]
fn read_character_test() {
    assert_eq!(Tokenizer::new(&['a'][..]).read_character(), None);
    assert_eq!(Tokenizer::new(&['"', 'a'][..]).read_character(), None);
    assert_eq!(
        Tokenizer::new(&['"', 'x', '"', 'a'][..]).read_character(),
        Some(PositionedToken(Token::Character('x'), 0))
    );
}

#[test]
fn read_space_test() {
    assert_eq!(Tokenizer::new(&['a', ' ', 'x'][..]).read_space(), None);
    assert_eq!(
        Tokenizer::new(&[' ', ' ', 'x'][..]).read_space(),
        Some(PositionedToken(Token::Space(2), 0))
    );
    assert_eq!(
        Tokenizer::new(&[' ', ' '][..]).read_space(),
        Some(PositionedToken(Token::Space(2), 0))
    );
}

// #[test]
// fn read_identifier_test() {
//     assert_eq!(
//         read_identifier(&['a', 'b', 'c', ' ', 'x'][..]),
//         (&[' ', 'x'][..], Some(String::from("abc")))
//     );
//     assert_eq!(
//         read_identifier(&['a', 'b', '3', ' ', '$'][..]),
//         (&[' ', '$'][..], Some(String::from("ab3")))
//     );
//     assert_eq!(
//         read_identifier(&['$', 'a', 'b', '3', ' '][..]),
//         (&['$', 'a', 'b', '3', ' '][..], None)
//     );
// }
