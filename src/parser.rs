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

#[derive(Debug, Clone)]
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
            1 => Ok(Box::new(seq.pop().unwrap())),
            _ => Ok(Box::new(Rule::Sequence(
                seq.into_iter().map(|r| Box::new(r)).collect(),
            ))),
        }
    }

    fn eat_element(&mut self) -> Result<Rule> {
        let mut left: Option<Rule> = None;
        while let [PositionedToken(token, _), ..] = self.tokens {
            if let Some(l) = &left {
                match token {
                    Token::Or => {
                        let mut rules = vec![Box::new(l.clone())];

                        self.bump(1); // Eat OR
                        let right = self.eat_element()?;
                        if let Rule::Or(v) = right {
                            rules.extend(v);
                        } else {
                            rules.push(Box::new(right));
                        }
                        left = Some(Rule::Or(rules));
                    }
                    Token::Exclude => {
                        self.bump(1); // Eat Exclude
                        let right = self.eat_element()?;
                        left = Some(Rule::Exclude {
                            from: Box::new(l.clone()),
                            target: Box::new(right),
                        });
                    }
                    _ => {
                        break;
                    }
                }
            } else {
                match token {
                    Token::GroupBegin => {
                        self.bump(1);
                        let inner = self.eat_rule()?;
                        if let [PositionedToken(Token::GroupEnd, _), ..] = self.tokens {
                            self.bump(1);
                            left = Some(Rule::Group(inner));
                        }
                    }
                    Token::RepeatBegin => {
                        self.bump(1);
                        let inner = self.eat_rule()?;
                        if let [PositionedToken(Token::RepeatEnd, _), ..] = self.tokens {
                            self.bump(1);
                            left = Some(Rule::Repeat(inner));
                        }
                    }
                    Token::OptionBegin => {
                        self.bump(1);
                        let inner = self.eat_rule()?;
                        if let [PositionedToken(Token::OptionEnd, _), ..] = self.tokens {
                            self.bump(1);
                            left = Some(Rule::Option(inner));
                        }
                    }
                    Token::Identifier(s) => {
                        self.bump(1);
                        left = Some(Rule::IdentifierRef(s.clone()));
                    }
                    Token::String(s) => {
                        self.bump(1);
                        left = Some(self.convert_string_rule(&s));
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
        if let Some(left) = left {
            Ok(left)
        } else {
            Err(self.make_error("Not valid rule"))
        }
    }

    pub fn convert_string_rule(&self, str: &str) -> Rule {
        Rule::Sequence(str.chars().map(|c| Box::new(Rule::Character(c))).collect())
    }
}
