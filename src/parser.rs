use crate::lexer::Token;

use super::lexer::PositionedToken;
use anyhow::Result;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [PositionedToken],
}

#[derive(Debug)]
pub struct Definition {
    pub identifier: String,
    pub rule: Box<Rule>,
}

#[derive(Debug)]
pub enum Rule {
    Character(char),
    IdentifierRef(String),
    Exclude { from: Box<Rule>, target: Box<Rule> },
    Sequence(Vec<Box<Rule>>),
    Or { left: Box<Rule>, right: Box<Rule> },
    Repeat(Box<Rule>),
    Option(Box<Rule>),
    Group(Box<Rule>),
}

impl<'a> Parser<'a> {
    pub fn new<'b>(tokens: &'b [PositionedToken]) -> Parser<'b> {
        Parser { tokens }
    }

    fn bump(&mut self, size: usize) {
        println!("Bump: {} {:?}", size, self.tokens[0]);
        self.tokens = &self.tokens[size..];
    }

    fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn eat(&mut self) -> Option<Vec<Box<Definition>>> {
        let mut defs = Vec::new();
        while let Some(def) = self.eat_single_def() {
            defs.push(def);
        }
        if self.is_empty() {
            Some(defs)
        } else {
            None
        }
    }

    fn eat_single_def(&mut self) -> Option<Box<Definition>> {
        if let [PositionedToken(Token::Identifier(identifier), pos), PositionedToken(Token::Equals, _), ..] =
            self.tokens
        {
            self.bump(2);
            if let Some(rule) = self.eat_rule() {
                if let [PositionedToken(Token::TokenEnd, pos), ..] = self.tokens {
                    self.bump(1);
                    return Some(Box::new(Definition {
                        identifier: identifier.clone(),
                        rule,
                    }));
                }
            }
        }
        None
    }

    fn eat_rule(&mut self) -> Option<Box<Rule>> {
        let mut seq = Vec::new();
        loop {
            if let Some(rule) = self.eat_element() {
                seq.push(rule);
            } else {
                return None;
            }
            if let [PositionedToken(token, pos), ..] = self.tokens {
                match token {
                    Token::Separator => {
                        self.bump(1);
                        continue;
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
        match seq.len() {
            0 => None,
            1 => Some(seq.pop().unwrap()),
            _ => Some(Box::new(Rule::Sequence(seq))),
        }
    }

    fn eat_element(&mut self) -> Option<Box<Rule>> {
        let mut seq = Vec::new();
        while let [PositionedToken(token, pos), ..] = self.tokens {
            match token {
                Token::Identifier(s) => {
                    self.bump(1);
                    seq.push(Box::new(Rule::IdentifierRef(s.clone())));
                }
                Token::String(s) => {
                    self.bump(1);
                    seq.push(self.convert_string_rule(&s));
                }
                Token::Or => {
                    if seq.len() < 1 {
                        return None;
                    }

                    self.bump(1); // Eat OR
                    if let Some(right) = self.eat_element() {
                        let rule = Rule::Or {
                            left: if seq.len() == 1 {
                                seq.pop().unwrap()
                            } else {
                                Box::new(Rule::Sequence(seq))
                            },
                            right,
                        };
                        seq = vec![Box::new(rule)];
                    }
                }
                Token::Exclude => {
                    if seq.len() != 1 {
                        return None;
                    }

                    self.bump(1); // Eat Exclude
                    if let Some(right) = self.eat_element() {
                        let rule = Rule::Exclude {
                            from: seq.pop().unwrap(),
                            target: right,
                        };
                        seq = vec![Box::new(rule)];
                    }
                }
                Token::GroupBegin => {
                    self.bump(1);
                    if let Some(inner) = self.eat_rule() {
                        if let [PositionedToken(Token::GroupEnd, pos), ..] = self.tokens {
                            self.bump(1);
                            seq.push(Box::new(Rule::Group(inner)));
                        }
                    }
                }
                Token::RepeatBegin => {
                    self.bump(1);
                    if let Some(inner) = self.eat_rule() {
                        if let [PositionedToken(Token::RepeatEnd, pos), ..] = self.tokens {
                            self.bump(1);
                            seq.push(Box::new(Rule::Repeat(inner)));
                        }
                    }
                }
                Token::OptionBegin => {
                    self.bump(1);
                    if let Some(inner) = self.eat_rule() {
                        if let [PositionedToken(Token::OptionEnd, pos), ..] = self.tokens {
                            self.bump(1);
                            seq.push(Box::new(Rule::Option(inner)));
                        }
                    }
                }
                _ => {
                    break;
                }
            }
        }
        match seq.len() {
            0 => None,
            1 => Some(seq.pop().unwrap()),
            _ => Some(Box::new(Rule::Sequence(seq))),
        }
    }

    pub fn convert_string_rule(&self, str: &str) -> Box<Rule> {
        Box::new(Rule::Sequence(
            str.chars().map(|c| Box::new(Rule::Character(c))).collect(),
        ))
    }
}
