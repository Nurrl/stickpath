use derive_more::Display;
use std::error::Error;

#[derive(Debug, Display, PartialEq)]
pub enum InputError {
    #[display(fmt = "An error occured while reading from standard input")]
    InError,

    #[display(fmt = "The input must be in the ASCII range")]
    IllegalChar,

    #[display(fmt = "The format of the first line must be '<num> <num>'")]
    FormatError,

    #[display(
        fmt = "The size constraints were not respected (w > 3 & h <= 100): ({}, {})",
        _0,
        _1
    )]
    SizeError(usize, usize),
}

impl Error for InputError {}
