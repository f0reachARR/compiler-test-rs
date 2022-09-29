use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GrammerIdentifier(pub u64);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Grammer {
    Empty, // 空語
    Dot,
    Grammer(GrammerIdentifier),
    Character(char),
}

pub type GrammerSet = HashMap<GrammerIdentifier, Vec<Vec<Grammer>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct GrammerAnnotation {
    pub endchars: HashSet<char>,
    pub identifiers: HashSet<u64>,
}

pub fn display_grammer_set(set: &GrammerSet) {
    for (id, grammers) in set {
        for grammer in grammers {
            print!("{} = ", id.0);
            for item in grammer {
                match item {
                    Grammer::Empty => {
                        print!("ε ");
                    }
                    Grammer::Dot => {
                        print!("* ");
                    }
                    Grammer::Grammer(GrammerIdentifier(id)) => {
                        print!("<{}> ", id);
                    }
                    Grammer::Character(c) => {
                        print!("'{}' ", c);
                    }
                }
            }
            println!("");
        }
    }
}

mod ebnf2gram;
mod utils;

pub use ebnf2gram::Ebnf2Gram;
pub use utils::{create_closure_set, create_first_set, create_follow_set};
