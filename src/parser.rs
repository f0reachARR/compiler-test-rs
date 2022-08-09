use crate::lexer::Token;

use super::lexer::PositionedToken;
use anyhow::Result;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [PositionedToken],
}

#[derive(Debug)]
pub enum Rule {
    Character(char),
    IdentifierRef(String),
    Exclude { from: Box<Rule>, target: Box<Rule> },
    Sequence(Vec<Box<Rule>>),
    Or(Vec<Box<Rule>>),
    Repeat(Box<Rule>),
    Option(Box<Rule>),
    Group(Box<Rule>),
}

impl<'a> Parser<'a> {
    pub fn new<'b>(tokens: &'b [PositionedToken]) -> Parser<'b> {
        Parser { tokens }
    }

    fn bump(&mut self, size: usize) {
        self.tokens = &self.tokens[size..];
    }

    fn eat(&mut self) -> Option<Rule> {
        None
    }

    fn eat_parentheses(&mut self) -> Option<Rule> {
        if let [PositionedToken(token, pos), ..] = self.tokens {
            match token {
                &Token::GroupBegin => {
                    self.bump(1);
                    if let Some(inner) = self.eat() {
                        if let [PositionedToken(Token::GroupEnd, pos), ..] = self.tokens {
                            self.bump(1);
                            return Some(Rule::Group(Box::new(inner)));
                        }
                    }
                }
                &Token::RepeatBegin => {
                    self.bump(1);
                    if let Some(inner) = self.eat() {
                        if let [PositionedToken(Token::RepeatEnd, pos), ..] = self.tokens {
                            self.bump(1);
                            return Some(Rule::Group(Box::new(inner)));
                        }
                    }
                }
                &Token::OptionBegin => {
                    self.bump(1);
                    if let Some(inner) = self.eat() {
                        if let [PositionedToken(Token::OptionEnd, pos), ..] = self.tokens {
                            self.bump(1);
                            return Some(Rule::Group(Box::new(inner)));
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn convert_string_rule(&self, str: &str) -> Rule {
        Rule::Sequence(str.chars().map(|c| Box::new(Rule::Character(c))).collect())
    }
}
