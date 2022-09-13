use std::collections::HashMap;

use super::grammer::{Grammer, GrammerIdentifier, GrammerSet};
use crate::parser::{Definition, Rule};
use anyhow::Result;

pub struct Ebnf2Gram {
    pub grammer_set: GrammerSet,
    pub identifier_map: HashMap<String, GrammerIdentifier>,
    identifier_counter: u64,
}

impl Ebnf2Gram {
    pub fn process(base: Vec<Box<Definition>>) -> Result<Self> {
        let identifier_map = base
            .iter()
            .enumerate()
            .map(|(i, x)| (x.identifier.clone(), GrammerIdentifier(i as u64)))
            .collect::<HashMap<_, _>>();
        let identifier_counter = identifier_map.len() as u64 + 1;

        let mut state = Self {
            grammer_set: HashMap::new(),
            identifier_map,
            identifier_counter,
        };

        for Definition { identifier, rule } in base.iter().map(|d| d.as_ref()) {
            let mut grammer = Vec::new();
            let selfref = state
                .identifier_map
                .get(identifier)
                .ok_or(anyhow::anyhow!("Unknown identifier"))?
                .clone();
            state.iterate(&mut grammer, rule.as_ref())?;
            state.grammer_set.entry(selfref).or_default().push(grammer);
        }

        Ok(state)
    }

    fn iterate(&mut self, grammer: &mut Vec<Grammer>, rule: &Rule) -> Result<()> {
        match rule {
            Rule::Character(c) => {
                grammer.push(Grammer::Character(*c));
            }
            Rule::IdentifierRef(i) => {
                let gref = self
                    .identifier_map
                    .get(i)
                    .ok_or(anyhow::anyhow!("Unknown identifier"))?;
                grammer.push(Grammer::Grammer(*gref));
            }
            Rule::Exclude { from, target } => todo!(),
            Rule::Sequence(inside_rule) => {
                for rule in inside_rule {
                    self.iterate(grammer, rule.as_ref())?;
                }
            }
            Rule::Or(rules) => {
                let next = self.next_identifier();
                for rule in rules {
                    let mut grammer: Vec<Grammer> = Vec::new();
                    self.iterate(&mut grammer, rule.as_ref())?;
                    self.grammer_set.entry(next).or_default().push(grammer);
                }
                grammer.push(Grammer::Grammer(next));
            }
            Rule::Repeat(_) => todo!(),
            Rule::Option(_) => todo!(),
            Rule::Group(_) => todo!(),
        }

        Ok(())
    }

    fn next_identifier(&mut self) -> GrammerIdentifier {
        let next = self.identifier_counter;
        self.identifier_counter += 1;
        GrammerIdentifier(next)
    }
}
