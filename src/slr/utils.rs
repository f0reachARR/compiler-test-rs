use std::collections::HashSet;

use super::grammer::{Grammer, GrammerSet};

pub fn create_first_set(set: &GrammerSet, char: &Grammer) -> HashSet<Grammer> {
    let mut first_set = HashSet::new();
    match char {
        Grammer::Empty => {
            first_set.insert(Grammer::Empty);
        }
        Grammer::Grammer(_) => {}
        Grammer::Character(c) => {
            first_set.insert(Grammer::Character(*c));
        }
    }
    first_set
}
