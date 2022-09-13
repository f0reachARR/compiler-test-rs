use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GrammerIdentifier(pub u64);

#[derive(Debug, PartialEq, Eq)]
pub enum Grammer {
    Empty, // 空語
    Grammer(GrammerIdentifier),
    Character(char),
}

pub type GrammerSet = HashMap<GrammerIdentifier, Vec<Vec<Grammer>>>;

pub fn display_grammer_set(set: &GrammerSet) {
    for (id, grammers) in set {
        for grammer in grammers {
            print!("{} = ", id.0);
            for item in grammer {
                match item {
                    Grammer::Empty => {
                        print!("ε ");
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
