use super::token::{Span, Token};

pub struct Lexer<'a> {
    src: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, cursor: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(Token<'a>, Span), ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let rest = &self.src[self.cursor..];

        if rest.is_empty() {
            return None;
        }

        match &rest[0..1] {
            "(" => {
                self.cursor += 1;
                Some(Ok((
                    Token::LeftParens,
                    Span::new(self.cursor, self.cursor + 1)
                )))
            }
            ")" => {
                self.cursor += 1;
                Some(Ok((
                    Token::RightParens,
                    Span::new(self.cursor - 1, self.cursor)
                )))
            }
            _ => unimplemented!(),
        }
    }
}
