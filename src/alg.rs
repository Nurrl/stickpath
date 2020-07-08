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

fn tokenizer(line: String, sep: &str) -> Result<Vec<char>> {
    line
        .split(sep)
        .map(|s| {
            if s.len() > 1 {
                return Err(InputError::MalformedInput);
            };
            Ok(s.chars().next().unwrap_or(0 as char))
        })
        .collect::<Result<Vec<char>>>()
}

fn processline(line: String, head: &mut Vec<char>) -> Result<()> {
    let connectors = tokenizer(line, "|")?;

    let mut swapped: bool = false;
    println!("{:?}", connectors);
    for (id, tok) in connectors.iter().enumerate() {
        /* If format is invalid */
        if (id == 0 || id == connectors.len() - 1) 
                && *tok != 0 as char {
            return Err(InputError::MalformedInput);
        } else if *tok == '-' {
            /* Two swaps in one line is invalid */
            if swapped { return Err(InputError::MalformedInput); }
            swapped = true;

            /* Swap the header to match with footer, according to pipes */
            head.swap(id - 1, id);
        }
    }
    Ok(())
}

pub fn retrieve() -> Result<(Vec<char>, Vec<char>)> {
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
    for id in 0..(h - 1) {
        let (line, len) = readline()?;

        /* If line is not as long as the it should */
        if len != w {
            return Err(InputError::MalformedInput);
        }
        if id == 0 {
            head = tokenizer(line, " ")?;
        } else {
            processline(line, &mut head)?;
        }
    }
    let (tail, _) = readline()?;
    let tail = tokenizer(tail, " ")?;

    Ok((head, tail))
}
