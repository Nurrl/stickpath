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

    #[display(fmt = "The size is mismatched: {} != {}", _0, _1)]
    SizeMismatch(usize, usize),

    #[display(fmt = "The identifiers at top and bottom must be unique")]
    UniqError,

    #[display(fmt = "The input is either malformed or doesn't provide enought informations")]
    MalformedInput,

    #[display(fmt = "Can't resolve adjacent pipes without breaking the fabric of the universe")]
    DoublePipe,
}

impl Error for InputError {}
