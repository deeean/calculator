#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  LeftParen,
  RightParen,

  Plus,
  Minus,
  Star,
  Slash,
  Percent,

  Number,

  Eof,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
  pub kind: TokenKind,
  pub slice: &'a str,
}

impl <'a> Token<'a> {
  pub fn new(kind: TokenKind, slice: &'a str) -> Self {
    Self {
      kind,
      slice,
    }
  }
}