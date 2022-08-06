use super::lexer::PositionedToken;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [PositionedToken],
}

#[derive(Debug)]
pub enum Rule {
    Character(char),
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

    pub fn extract_rule(&mut self) -> Vec<Rule> {
        let rules = Vec::new();
        while let [token, rest @ ..] = self.tokens {}
        rules
    }

    pub fn convert_string_rule(str: &str) -> Vec<Rule> {
        str.chars().map(|c| Rule::Character(c)).collect()
    }
}
