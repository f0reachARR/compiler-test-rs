#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Space(usize),
    Equals,
    String(String),
    GroupBegin,
    GroupEnd,
    RepeatBegin,
    RepeatEnd,
    OptionBegin,
    OptionEnd,
    Or,
    Exclude,
    Separator,
    TokenEnd,
    LineEnd,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PositionedToken(pub Token, pub usize);

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a [char],
    cursor: &'a [char],
}

impl<'a> Tokenizer<'a> {
    pub fn new<'b>(input: &'b [char]) -> Tokenizer<'b> {
        Tokenizer {
            input,
            cursor: input,
        }
    }

    fn get_pos(&self) -> usize {
        self.input.len() - self.cursor.len()
    }

    pub fn read_definition(mut self) -> Option<Vec<PositionedToken>> {
        let mut tokens = Vec::new();
        loop {
            if let Some(t) = self.read_string() {
                tokens.push(t);
            } else if let Some(t) = self.read_identifier() {
                tokens.push(t);
            } else if let Some(t) = self.read_equals() {
                tokens.push(t);
            } else if let Some(t) = self.read_single_token() {
                tokens.push(t);
            } else if let Some(_) = self.read_space() {
                // tokens.push(t);
            } else if let Some(_) = self.read_lineend() {
                // tokens.push(t);
            } else {
                break;
            }
        }

        if self.cursor.len() > 0 {
            None
        } else {
            Some(tokens)
        }
    }

    fn read_string(&mut self) -> Option<PositionedToken> {
        let pos = self.get_pos();
        if let [start @ ('\'' | '"'), rest @ ..] = self.cursor {
            self.cursor = rest;
            let mut str = String::new();
            while let [c, rest @ ..] = self.cursor {
                self.cursor = rest;
                if *c != *start {
                    str.push(*c);
                } else {
                    return Some(PositionedToken(Token::String(str), pos));
                }
            }
        }
        None
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
        let mut cursor = self.cursor;
        while let [c @ ('0'..='9' | 'a'..='z' | 'A'..='Z' | ' '), rest @ ..] = cursor {
            identifier.push(*c);
            cursor = rest;
        }
        identifier = identifier.trim().to_string();
        if identifier.is_empty() {
            None
        } else {
            self.cursor = cursor;
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
            ['|', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::Or, pos))
            }
            ['-', rest @ ..] => {
                self.cursor = rest;
                Some(PositionedToken(Token::Exclude, pos))
            }
            _ => None,
        }
    }
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

#[test]
fn read_string_test() {
    assert_eq!(
        Tokenizer::new(&['\'', 'x', 'x', '\''][..]).read_string(),
        Some(PositionedToken(Token::String(String::from("xx")), 0))
    );
    assert_eq!(
        Tokenizer::new(&['\'', '"', 'x', '\''][..]).read_string(),
        Some(PositionedToken(Token::String(String::from("\"x")), 0))
    );
    assert_eq!(
        Tokenizer::new(&['\'', 'x', 'x', '"'][..]).read_string(),
        None
    );
    assert_eq!(
        Tokenizer::new(&['"', 'x', 'x', '\''][..]).read_string(),
        None
    );
    assert_eq!(Tokenizer::new(&['"', 'x'][..]).read_string(), None);
}
