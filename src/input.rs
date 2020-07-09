use std::collections::HashSet;
use std::hash::Hash;
use std::io;

use crate::error::InputError;

type Result<T> = std::result::Result<T, InputError>;

macro_rules! parse {
    ($string:expr, $sep:expr, $err:expr, $($x:ty),+) => {{
        let mut iter = $string.split($sep);
        let ret = ($(
            match iter.next() {
                None => return Err($err),
                Some(word) => match word.parse::<$x>().ok() {
                    None => return Err($err),
                    Some(data) => data
                }
            },)*);
        if iter.next().is_some() { return Err($err) }
        ret
    }}
}

fn readline() -> Result<(String, usize)> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err(InputError::InError),
    }

    let trimmed = buf.trim().to_string();
    let count = trimmed.chars().count();

    /* Restrict whole input to ascii */
    if !trimmed.is_ascii() {
        return Err(InputError::IllegalChar);
    }

    Ok((trimmed, count))
}

fn tokenizer(line: String, sep: fn(char) -> bool, allowmultiple: bool) -> Result<Vec<char>> {
    line.split(sep)
        .filter(|x| allowmultiple || x.len() > 0)
        .map(|s| {
            if !allowmultiple && s.len() > 1 {
                return Err(InputError::MalformedInput);
            };
            Ok(s.chars().next().unwrap_or(0 as char))
        })
        .collect::<Result<Vec<char>>>()
}

fn processline(line: String, pipes: &mut Vec<(usize, usize)>) -> Result<()> {
    let connectors = tokenizer(line, |x| x == '|', true)?;

    let mut piped: bool = false;
    for (id, tok) in connectors.iter().enumerate() {
        /* If format is invalid */
        if id == 0 || id == connectors.len() - 1 {
            if *tok != 0 as char {
                return Err(InputError::MalformedInput);
            }
        } else if *tok == '-' {
            /* Two pipes in a row is invalid */
            if piped {
                return Err(InputError::DoublePipe);
            }
            piped = true;

            /* Provide pipes with indexes on what to swap */
            pipes.push((id - 1, id));
        } else if char::is_whitespace(*tok) {
            piped = false;
        } else {
            /* Other than '-' or `whitespace` is invalid */
            return Err(InputError::MalformedInput);
        }
    }

    Ok(())
}

fn isuniq<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn retrieve() -> Result<(Vec<char>, Vec<(usize, usize)>, Vec<char>)> {
    let (params, _) = readline()?;
    let (w, h) = parse!(
        params,
        char::is_whitespace,
        InputError::FormatError,
        usize,
        usize
    );

    /* Check for size contraints */
    if w <= 3 || h > 100 {
        return Err(InputError::SizeError(w, h));
    }
    if h < 3 {
        return Err(InputError::MalformedInput);
    }

    /* Process each line */
    let mut head: Vec<char> = Vec::new();
    let mut pipes: Vec<(usize, usize)> = Vec::new();
    let mut tail: Vec<char> = Vec::new();
    for id in 0..h {
        let (line, len) = readline()?;

        /* If line is not as long as the it should */
        if len != w {
            return Err(InputError::SizeMismatch(len, w));
        }
        if id == 0 {
            /* If head */
            head = tokenizer(line, char::is_whitespace, false)?;
        } else if id == h - 1 {
            /* If tail */
            tail = tokenizer(line, char::is_whitespace, false)?;
        } else {
            /* If pipes */
            processline(line, &mut pipes)?;
        }
    }
    /* Check for uniqueness of identifiers */
    let concat = [&head[..], &tail[..]].concat();
    if !isuniq(concat) {
        return Err(InputError::UniqError);
    }

    Ok((head, pipes, tail))
}
