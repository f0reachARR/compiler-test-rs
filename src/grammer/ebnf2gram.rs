use std::collections::{HashMap, HashSet};

use super::{
    create_first_set, create_follow_set, Grammer, GrammerAnnotation, GrammerIdentifier, GrammerSet,
};
use crate::parser::{Definition, Rule};
use anyhow::Result;

pub struct Ebnf2Gram {
    grammer_set: GrammerSet,
    identifier_map: HashMap<String, GrammerIdentifier>,
    end_characters: HashSet<char>,
    identifiers: HashSet<u64>,
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
            end_characters: HashSet::new(),
            identifiers: identifier_map.iter().map(|(_, x)| x.0).collect(),
            identifier_map,
            identifier_counter,
        };

        for Definition { identifier, rule } in base.iter().map(|d| d.as_ref()) {
            let mut grammer = Vec::new();
            let selfref = state
                .identifier_map
                .get(identifier)
                .ok_or(anyhow::anyhow!("Unknown identifier {}", identifier))?
                .clone();
            state.iterate(&mut grammer, rule.as_ref())?;
            state.grammer_set.entry(selfref).or_default().push(grammer);
        }

        Ok(state)
    }

    fn iterate(&mut self, grammer: &mut Vec<Grammer>, rule: &Rule) -> Result<()> {
        match rule {
            Rule::Character(c) => {
                self.end_characters.insert(*c);
                grammer.push(Grammer::Character(*c));
            }
            Rule::IdentifierRef(i) => {
                let gref = self
                    .identifier_map
                    .get(i)
                    .ok_or(anyhow::anyhow!("Unknown identifier {}", i))?;
                grammer.push(Grammer::Grammer(*gref));
            }
            Rule::Exclude { from: _, target: _ } => todo!(),
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
            Rule::Repeat(rule) => {
                let target_id = self.next_identifier();
                let wrap_id = self.next_identifier();
                // Target grammer
                let mut new_grammer: Vec<Grammer> = Vec::new();
                self.iterate(&mut new_grammer, rule.as_ref())?;
                new_grammer.push(Grammer::Grammer(wrap_id.clone()));
                self.grammer_set
                    .entry(target_id)
                    .or_default()
                    .push(new_grammer);

                // Ref grammer
                let new_grammer = vec![Grammer::Empty];
                self.grammer_set
                    .entry(wrap_id)
                    .or_default()
                    .push(new_grammer);
                let new_grammer = vec![Grammer::Grammer(target_id)];
                self.grammer_set
                    .entry(wrap_id)
                    .or_default()
                    .push(new_grammer);

                grammer.push(Grammer::Grammer(wrap_id));
            }
            Rule::Option(rule) => {
                let target_id = self.next_identifier();
                let wrap_id = self.next_identifier();
                // Once grammer grammer
                let mut new_grammer: Vec<Grammer> = Vec::new();
                self.iterate(&mut new_grammer, rule.as_ref())?;
                self.grammer_set
                    .entry(target_id)
                    .or_default()
                    .push(new_grammer);
                // Ref grammer
                let new_grammer = vec![Grammer::Empty];
                self.grammer_set
                    .entry(wrap_id)
                    .or_default()
                    .push(new_grammer);
                let new_grammer = vec![Grammer::Grammer(target_id)];
                self.grammer_set
                    .entry(wrap_id)
                    .or_default()
                    .push(new_grammer);

                grammer.push(Grammer::Grammer(wrap_id));
            }
            Rule::Group(rule) => {
                let next = self.next_identifier();
                // Once grammer grammer
                let mut new_grammer: Vec<Grammer> = Vec::new();
                self.iterate(&mut new_grammer, rule.as_ref())?;
                self.grammer_set.entry(next).or_default().push(new_grammer);

                grammer.push(Grammer::Grammer(next));
            }
        }

        Ok(())
    }

    fn next_identifier(&mut self) -> GrammerIdentifier {
        let next = self.identifier_counter;
        self.identifier_counter += 1;
        self.identifiers.insert(next);
        GrammerIdentifier(next)
    }

    pub fn get_grammer_set(&self) -> &GrammerSet {
        &self.grammer_set
    }

    pub fn get_identifier_map(&self) -> &HashMap<String, GrammerIdentifier> {
        &self.identifier_map
    }

    pub fn create_annotations(&self) -> GrammerAnnotation {
        let mut first_set = HashMap::new();

        for char in self.end_characters.iter() {
            let first = create_first_set(&self.grammer_set, &Grammer::Character(*char));
            first_set.insert(Grammer::Character(*char), first);
        }

        for id in self.identifiers.iter() {
            let first =
                create_first_set(&self.grammer_set, &Grammer::Grammer(GrammerIdentifier(*id)));
            first_set.insert(Grammer::Grammer(GrammerIdentifier(*id)), first);
        }

        let follow = create_follow_set(&self.grammer_set);

        GrammerAnnotation {
            endchars: self.end_characters.clone(),
            identifiers: self.identifiers.clone(),
            first_set,
            follow_set: follow,
        }
    }
}
