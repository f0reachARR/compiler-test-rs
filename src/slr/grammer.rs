#[derive(Debug, PartialEq, Eq)]
pub struct GrammerIdentifier(pub u64);

#[derive(Debug, PartialEq, Eq)]
pub enum Grammer {
    Empty, // 空語
    Grammer(GrammerIdentifier),
    Character(char),
}
