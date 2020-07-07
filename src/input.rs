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

pub fn retrieve() -> Result<String> {
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
    Ok(String::from(""))
}
