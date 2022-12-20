use std::{
    cmp::Ordering::{self, *},
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    ops::Index,
};

fn main() {
    compare_sockets_from_file("./input.txt");
    //compare_bytes_from_file("./input.txt");
}

fn compare_sockets_from_file(file_path: &str) -> bool {
    let socket_file = File::open(file_path).unwrap();
    let reader = BufReader::new(socket_file);

    let mut push_to_a = true;

    let mut packets_a = Vec::new();
    let mut packets_b = Vec::new();

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        if push_to_a {
            packets_a.push(line.into_bytes());
            push_to_a = !push_to_a
        } else {
            packets_b.push(line.into_bytes());
            push_to_a = !push_to_a
        }
    }

    packets_b.push(b"[[2]]".to_vec());
    packets_b.push(b"[[6]]".to_vec());

    let mut all: Vec<Vec<Token>> = packets_a
        .into_iter()
        .chain(packets_b.into_iter())
        .map(|el| tokenize_int_array(String::from_utf8(el).unwrap()).unwrap())
        .collect();

    all.sort_by(|a, b| compare_tokens(&mut a.clone(), &mut b.clone()));

    all.reverse();

    let find_index: Vec<(usize, Vec<Token>)> = all
        .into_iter()
        .enumerate()
        .filter(|(i, v)| {
            compare_tokens(
                &mut v.clone(),
                &mut tokenize_int_array(String::from_utf8(b"[[2]]".to_vec()).unwrap()).unwrap(),
            ) == Ordering::Equal
                || compare_tokens(
                    &mut v.clone(),
                    &mut tokenize_int_array(String::from_utf8(b"[[6]]".to_vec()).unwrap()).unwrap(),
                ) == Ordering::Equal
        })
        .collect();

    println!("{find_index:?}");

    println!("{}", find_index.iter().fold(1, |acc, v| acc * (v.0 + 1)));

    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsingError {
    kind: ParsingKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsingKind {
    IntParseError,
    UnknownToken,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Value(i32),
    Open,
    Close,
}

fn tokenize_int_array(s: String) -> Result<Vec<Token>, ParsingError> {
    let mut tokens = Vec::new();
    let mut element: Option<i32> = None;
    for c in s.chars() {
        match c {
            '[' => {
                if let Some(el) = element.take() {
                    tokens.push(Token::Value(el))
                }
                tokens.push(Token::Open)
            }
            ']' => {
                if let Some(el) = element.take() {
                    tokens.push(Token::Value(el))
                }
                tokens.push(Token::Close)
            }
            ',' => {
                if let Some(el) = element.take() {
                    tokens.push(Token::Value(el))
                }
                continue;
            }
            ' ' => {
                continue;
            }
            digit if digit.is_numeric() => match element {
                Some(mut v) => {
                    v *= 10;
                    let x = digit.to_digit(10).ok_or(ParsingError {
                        kind: ParsingKind::IntParseError,
                    })?;
                    element = Some(v + x as i32)
                }
                None => {
                    element = Some(digit.to_digit(10).ok_or(ParsingError {
                        kind: ParsingKind::IntParseError,
                    })? as i32)
                }
            },
            _ => {
                return Err(ParsingError {
                    kind: ParsingKind::UnknownToken,
                })
            }
        }
    }
    Ok(tokens)
}

fn compare_tokens(a: &mut Vec<Token>, b: &mut Vec<Token>) -> Ordering {
    let mut index_a = 0;
    let mut index_b = 0;
    loop {
        match (a.get(index_a), b.get(index_b)) {
            // both lists are finished
            (None, None) => return Ordering::Equal,
            // left finished first
            (None, Some(_)) => return Ordering::Greater,
            // right finished first
            (Some(_), None) => return Ordering::Less,
            // Both Lists have a token compare them
            (Some(t_a), Some(t_b)) => match (t_a, t_b) {
                // Both sides start new list do nothing
                (a, b) if *a == *b => {}
                // Value Token compare
                (Token::Value(v_a), Token::Value(v_b)) => {
                    if *v_a < *v_b {
                        return Ordering::Greater;
                    } else if *v_a > *v_b {
                        return Ordering::Less;
                    }
                }
                // Right has list left has Value
                (Token::Value(_), Token::Open) => {
                    a.insert(index_a, Token::Open);
                    a.insert(index_a + 2, Token::Close);
                }
                // Left has list right has Value
                (Token::Open, Token::Value(_)) => {
                    b.insert(index_b, Token::Open);
                    b.insert(index_b + 2, Token::Close);
                }
                // Left list was shorter walk right index until list closes
                (Token::Close, _) => return Ordering::Greater,
                (_, Token::Close) => return Ordering::Less,
                (Token::Open, Token::Open) => {}
            },
        }
        index_a += 1;
        index_b += 1;
    }
}
