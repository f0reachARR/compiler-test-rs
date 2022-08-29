use std::collections::HashMap;

use super::grammer::{Grammer, GrammerIdentifier};
use crate::parser::Definition;
use anyhow::Result;

fn ebnf2gram(ebnf: Vec<Box<Definition>>) -> Result<HashMap<GrammerIdentifier, Grammer>> {}
