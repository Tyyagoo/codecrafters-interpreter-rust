use std::fmt::Display;

use logos::Logos;
use Token::*;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[\s\t\n\f]+")]
pub enum Token {
    #[token("(")]
    LeftParens,
    #[token(")")]
    RightParens,
    #[token("{")]
    LeftBraces,
    #[token("}")]
    RightBraces,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftParens => write!(f, "LEFT_PAREN ( null"),
            RightParens => write!(f, "RIGHT_PAREN ) null"),
            LeftBraces => write!(f, "LEFT_BRACE {{ null"),
            RightBraces => write!(f, "RIGHT_BRACE }} null"),
        }
    }
}