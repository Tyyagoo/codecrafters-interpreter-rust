use std::fmt::Display;
use Token::*;

pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

pub enum Token<'a> {
    LeftParens,
    RightParens,
    Id(&'a str),
    EOF,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftParens => write!(f, "LEFT_PAREN ( null"),
            RightParens => write!(f, "RIGHT_PAREN ) null"),
            EOF => write!(f, "EOF  null"),
            _ => unimplemented!(),
        }
    }
}