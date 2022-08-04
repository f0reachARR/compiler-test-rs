use std::{fs::File, io::Read};

#[derive(Debug)]
enum Token {
    Identifier(String),
    Space,
    Equals,
    Character(char),
    GroupBegin,
    GroupEnd,
    RepeatBegin,
    RepeatEnd,
    OptionBegin,
    OptionEnd,
    Separator,
}

fn main() {
    let input = std::env::args().nth(1).expect("no input file");
    let mut file = File::open(input).expect("input file open");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // let input_vec: Vec<char> = &buf.as_str().chars().collect::<Vec<_>>();
    let mut input_buf: &[char] = &buf.as_str().chars().collect::<Vec<_>>();

    while !input_buf.is_empty() {
        let (rest, identifier) = read_identifier(input_buf);
        if let Some(identifierstr) = identifier {
            println!("id: {}", identifierstr);
        } else {
            break;
        }
        input_buf = rest;

        input_buf = read_space(input_buf);

        let (rest, equal) = read_equals(input_buf);
        if !equal {
            break;
        }
        input_buf = rest;

        input_buf = read_space(input_buf);

        loop {
            let (rest, c) = read_character(input_buf);
            if let Some(c) = c {
                println!("c: {}", c);
            } else {
                break;
            }
            input_buf = rest;

            input_buf = read_space(input_buf);
        }
    }
}

fn read_equals(mut input: &[char]) -> (&[char], bool) {
    if let ['=', rest @ ..] = input {
        input = rest;
        (input, true)
    } else if let [':', ':', '=', rest @ ..] = input {
        input = rest;
        (input, true)
    } else {
        (input, false)
    }
}

#[test]
fn read_equals_test() {
    assert_eq!(read_equals(&['"', 'x'][..]), (&['"', 'x'][..], false));
    assert_eq!(read_equals(&['=', 'x'][..]), (&['x'][..], true));
    assert_eq!(read_equals(&[':', ':', '=', 'x'][..]), (&['x'][..], true));
}

fn read_character(mut input: &[char]) -> (&[char], Option<char>) {
    if let ['"', c, '"', rest @ ..] = input {
        input = rest;
        (input, Some(*c))
    } else {
        (input, None)
    }
}

#[test]
fn read_character_test() {
    assert_eq!(
        read_character(&"x".chars().collect::<Vec<_>>()),
        (&['x'][..], None)
    );
    assert_eq!(read_character(&['"', 'x'][..]), (&['"', 'x'][..], None));
    assert_eq!(
        read_character(&['"', 'x', '"', 'a'][..]),
        (&['a'][..], Some('x'))
    );
}

fn read_space(mut input: &[char]) -> &[char] {
    while let [' ', rest @ ..] = input {
        input = rest;
    }
    input
}

#[test]
fn read_space_test() {
    assert_eq!(read_space(&[' ', ' ', 'x'][..]), &['x'][..]);
    assert_eq!(read_space(&['a', ' ', 'x'][..]), &['a', ' ', 'x'][..]);
    assert_eq!(read_space(&[' ', ' '][..]), &[][..]);
}

fn read_identifier(mut input: &[char]) -> (&[char], Option<String>) {
    let mut identifier = String::new();
    while let [c @ ('0'..='9' | 'a'..='z' | 'A'..='Z'), rest @ ..] = input {
        identifier.push(*c);
        input = rest;
    }
    if identifier.is_empty() {
        (input, None)
    } else {
        (input, Some(identifier))
    }
}

#[test]
fn read_identifier_test() {
    assert_eq!(
        read_identifier(&['a', 'b', 'c', ' ', 'x'][..]),
        (&[' ', 'x'][..], Some(String::from("abc")))
    );
    assert_eq!(
        read_identifier(&['a', 'b', '3', ' ', '$'][..]),
        (&[' ', '$'][..], Some(String::from("ab3")))
    );
    assert_eq!(
        read_identifier(&['$', 'a', 'b', '3', ' '][..]),
        (&['$', 'a', 'b', '3', ' '][..], None)
    );
}
