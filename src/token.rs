use std::fmt::Display;

use logos::{Lexer, Logos, Skip, Span};
use Token::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    #[default]
    Other,
    UnexpectedCharacter(usize, Span),
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
#[logos(extras = usize)]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
    #[token("(")]
    LeftParens,
    #[token(")")]
    RightParens,
    #[token("{")]
    LeftBraces,
    #[token("}")]
    RightBraces,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("*")]
    Star,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[regex(r"\n", newline_callback)]
    Newline,
    #[regex(r".", priority=1, callback=unexpected_character_callback)]
    UnexpectedCharacter,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftParens => write!(f, "LEFT_PAREN ( null"),
            RightParens => write!(f, "RIGHT_PAREN ) null"),
            LeftBraces => write!(f, "LEFT_BRACE {{ null"),
            RightBraces => write!(f, "RIGHT_BRACE }} null"),
            Dot => write!(f, "DOT . null"),
            Minus => write!(f, "MINUS - null"),
            Plus => write!(f, "PLUS + null"),
            Star => write!(f, "STAR * null"),
            Comma => write!(f, "COMMA , null"),
            Semicolon => write!(f, "SEMICOLON ; null"),
            _ => unimplemented!(),
        }
    }
}

fn unexpected_character_callback(lexer: &mut Lexer<Token>) -> Result<Token, LexingError> {
    Err(LexingError::UnexpectedCharacter(lexer.extras, lexer.span()))
}

fn newline_callback(lexer: &mut Lexer<Token>) -> Skip {
    lexer.extras += 1;
    Skip
}

pub struct Scanner<'a> {
    inner: Lexer<'a, Token>,
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            inner: Token::lexer_with_extras(source, 1),
        }
    }

    pub fn next(&mut self) -> Option<Result<Token, String>> {
        self.inner.next().map(|maybe_token| {
            maybe_token.map_err(|err| match err {
                LexingError::UnexpectedCharacter(line, span) => format!(
                    "[line {}] Error: Unexpected character: {}",
                    line, &self.source[span]
                ),
                LexingError::Other => "Oh fuck.".to_owned(),
            })
        })
    }
}
