use std::collections::{HashMap, HashSet};

use super::{Grammer, GrammerIdentifier, GrammerSet};

pub fn create_first_set(set: &GrammerSet, char: &Grammer) -> HashSet<Grammer> {
    let mut first_set = HashSet::new();
    match char {
        Grammer::Empty => {
            first_set.insert(Grammer::Empty);
        }
        Grammer::Grammer(id) => {
            for grammer in set.get(id).unwrap() {
                for char in grammer {
                    let first = create_first_set(set, char);
                    // :thinking_face:
                    if first.contains(&Grammer::Empty) {
                        first_set.extend(first.into_iter());
                    } else {
                        first_set.extend(first.into_iter());
                        break;
                    }
                }
            }
        }
        Grammer::Character(c) => {
            first_set.insert(Grammer::Character(*c));
        }
        Grammer::Dot => {
            panic!("Unexpected dot");
        }
    }
    first_set
}

pub fn create_follow_set(set: &GrammerSet) -> HashMap<GrammerIdentifier, HashSet<Grammer>> {
    let mut follow_set_map: HashMap<GrammerIdentifier, HashSet<Grammer>> = HashMap::new();

    loop {
        let mut found = false;
        for (grammer_id, grammers) in set {
            for grammer in grammers {
                for i in 0..grammer.len() {
                    if let Some(Grammer::Grammer(target_id)) = grammer.get(i) {
                        let target_map = follow_set_map.entry(*target_id).or_default();
                        let mut has_nonempty = false;
                        if let Some(remain) = grammer.get(i + 1..grammer.len()) {
                            for next in remain {
                                match next {
                                    Grammer::Character(c) => {
                                        found |= !target_map.contains(&Grammer::Character(*c));

                                        target_map.insert(Grammer::Character(*c));
                                        has_nonempty = true;
                                        break;
                                    }
                                    Grammer::Empty => {
                                        panic!("Empty word not allowed between chars")
                                    }
                                    Grammer::Dot => panic!("Dot not allowed"),
                                    Grammer::Grammer(next_id) => {
                                        let first =
                                            create_first_set(set, &Grammer::Grammer(*next_id));
                                        found |= !target_map.is_superset(&first);
                                        if first.contains(&Grammer::Empty) {
                                            target_map.extend(first);
                                        } else {
                                            target_map.extend(first);
                                            has_nonempty = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if !has_nonempty {
                            let cloned = follow_set_map.entry(*grammer_id).or_default().clone();
                            // TODO: Re-borrow
                            let target_map = follow_set_map.entry(*target_id).or_default();
                            found |= !target_map.is_superset(&cloned);
                            target_map.extend(cloned);
                        }
                    }
                }
            }
        }

        if !found {
            break;
        }
    }
    follow_set_map
}

// 再帰かなあ
pub fn create_closure_set(set: &GrammerSet, input: &GrammerSet) -> GrammerSet {
    let mut output = input.clone();
    for (id, grammers) in input {
        for grammer in grammers {
            for i in 0..grammer.len() - 1 {
                if grammer[i] != Grammer::Dot {
                    continue;
                }
                if let Grammer::Grammer(ref_id) = grammer[i + 1] {}
            }
        }
    }
    output
}
