#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  LeftParen,
  RightParen,

  Comma,

  Plus,
  Minus,
  Star,
  Slash,
  Percent,

  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  Pipe,
  PipePipe,

  Amp,
  AmpAmp,

  Number,
  Boolean,
  Identifier,

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