use core::fmt;
use std::{error::Error, fmt::Display};

pub mod c;
pub mod cpp;
pub mod nix;
pub mod python;
pub mod rust;

#[derive(Debug)]
pub struct MkboilerError(String);

impl MkboilerError {
    pub fn new(value: String) -> Self {
        MkboilerError(value)
    }
}

impl Display for MkboilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MkboilerError {}

pub enum IndentStyle {
    Tabs,
    Spaces(u32),
}

pub struct Config<'a> {
    pub target: &'a str,
    pub output: Option<&'a str>,
    pub indent: IndentStyle,
}
