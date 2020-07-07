use derive_more::Display;
use std::error::Error;

#[derive(Debug, Display, PartialEq)]
pub enum InputError {}

impl Error for InputError {}
