use crate::lexer::Token;

use super::lexer::PositionedToken;
use anyhow::{anyhow, Result};

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
        // println!("Bump: {} {:?}", size, self.tokens[0]);
        self.tokens = &self.tokens[size..];
    }

    fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    fn make_error(&self, msg: &str) -> anyhow::Error {
        if let [PositionedToken(_, pos), ..] = self.tokens {
            anyhow::Error::msg(format!("{} {}", pos, msg.to_string()))
        } else {
            anyhow::Error::msg(format!("Unknown position {}", msg.to_string()))
        }
    }

    pub fn eat(&mut self) -> Result<Vec<Box<Definition>>> {
        let mut defs = Vec::new();
        while !self.is_empty() {
            defs.push(self.eat_single_def()?);
        }
        if self.is_empty() {
            Ok(defs)
        } else {
            Err(anyhow!("No rule"))
        }
    }

    fn eat_single_def(&mut self) -> Result<Box<Definition>> {
        if let [PositionedToken(Token::Identifier(identifier), _), PositionedToken(Token::Equals, _), ..] =
            self.tokens
        {
            self.bump(2);
            let rule = self.eat_rule()?;
            if let [PositionedToken(Token::TokenEnd, _), ..] = self.tokens {
                self.bump(1);
                return Ok(Box::new(Definition {
                    identifier: identifier.clone(),
                    rule,
                }));
            }
        }
        Err(self.make_error("Definition is not valid"))
    }

    fn eat_rule(&mut self) -> Result<Box<Rule>> {
        let mut seq = Vec::new();
        loop {
            let rule = self.eat_element()?;
            seq.push(rule);
            if let [PositionedToken(token, _), ..] = self.tokens {
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
            0 => Err(self.make_error("No rule found between separator")),
            1 => Ok(seq.pop().unwrap()),
            _ => Ok(Box::new(Rule::Sequence(seq))),
        }
    }

    fn eat_element(&mut self) -> Result<Box<Rule>> {
        let mut seq = Vec::new();
        while let [PositionedToken(token, _), ..] = self.tokens {
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
                        return Err(self.make_error("No left hand on OR operator"));
                    }

                    self.bump(1); // Eat OR
                    let right = self.eat_element()?;
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
                Token::Exclude => {
                    if seq.len() != 1 {
                        return Err(self.make_error("No left hand on Exclude operator"));
                    }

                    self.bump(1); // Eat Exclude
                    let right = self.eat_element()?;
                    let rule = Rule::Exclude {
                        from: seq.pop().unwrap(),
                        target: right,
                    };
                    seq = vec![Box::new(rule)];
                }
                Token::GroupBegin => {
                    self.bump(1);
                    let inner = self.eat_rule()?;
                    if let [PositionedToken(Token::GroupEnd, _), ..] = self.tokens {
                        self.bump(1);
                        seq.push(Box::new(Rule::Group(inner)));
                    }
                }
                Token::RepeatBegin => {
                    self.bump(1);
                    let inner = self.eat_rule()?;
                    if let [PositionedToken(Token::RepeatEnd, _), ..] = self.tokens {
                        self.bump(1);
                        seq.push(Box::new(Rule::Repeat(inner)));
                    }
                }
                Token::OptionBegin => {
                    self.bump(1);
                    let inner = self.eat_rule()?;
                    if let [PositionedToken(Token::OptionEnd, _), ..] = self.tokens {
                        self.bump(1);
                        seq.push(Box::new(Rule::Option(inner)));
                    }
                }
                _ => {
                    break;
                }
            }
        }
        match seq.len() {
            0 => Err(self.make_error("No element found")),
            1 => Ok(seq.pop().unwrap()),
            _ => Ok(Box::new(Rule::Sequence(seq))),
        }
    }

    pub fn convert_string_rule(&self, str: &str) -> Box<Rule> {
        Box::new(Rule::Sequence(
            str.chars().map(|c| Box::new(Rule::Character(c))).collect(),
        ))
    }
}
