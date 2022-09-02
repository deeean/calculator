use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
  input: &'a str,
  curr: usize,
  next: usize,
}

impl <'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input,
      curr: 0,
      next: 1,
    }
  }

  fn is_ended(&self) -> bool {
    self.curr >= self.input.len()
  }

  fn peek(&self) -> u8 {
    if self.is_ended() {
      b'\0'
    } else {
      self.input.as_bytes()[self.curr]
    }
  }

  fn next(&mut self) -> u8 {
    let ch = self.peek();
    self.curr = self.next;
    self.next += 1;
    ch
  }

  fn skip_whitespace(&mut self) {
    loop {
      match self.peek() {
        b' ' | b'\n' | b'\r' | b'\t' => {
          self.next();
        }
        _ => break,
      }
    }
  }

  fn read_number(&mut self) -> Token<'a> {
    let start = self.curr;

    loop {
      match self.peek() {
        b'0'..=b'9' => {
          self.next();
        }
        _ => {
          break;
        }
      }
    }

    if self.peek() == b'.' {
      self.next();

      loop {
        match self.peek() {
          b'0'..=b'9' => {
            self.next();
          }
          _ => {
            break;
          }
        }
      }
    }

    let slice = &self.input[start..self.curr];

    Token::new(TokenKind::Number, slice)
  }

  fn token(&mut self) -> Token<'a> {
    self.skip_whitespace();

    let curr = self.curr;

    let kind = match self.peek() {
      b'0'..=b'9' => {
        return self.read_number();
      }
      b'+' => {
        TokenKind::Plus
      }
      b'-' => {
        TokenKind::Minus
      }
      b'*' => {
        TokenKind::Star
      }
      b'/' => {
        TokenKind::Slash
      }
      b'%' => {
        TokenKind::Percent
      }
      b'(' => {
        TokenKind::LeftParen
      }
      b')' => {
        TokenKind::RightParen
      }
      b'\0' => {
        return Token::new(TokenKind::Eof, "");
      }
      _ => {
        panic!("Unexpected character: {}", self.peek() as char)
      },
    };

    let slice = &self.input[curr..self.next];
    self.next();
    Token::new(kind, slice)
  }

  fn lex(&mut self) {
    let mut tokens = Vec::new();

    loop {
      let token = self.token();
      tokens.push(token);

      if let Some(last_token) = tokens.last() {
        if last_token.kind == TokenKind::Eof {
          break;
        }
      }

      println!("{:?}", self.peek() as char);
    }

    println!("{:#?}", tokens);
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn lexer() {
    let mut lexer = super::Lexer::new("(3.141592 + 20) - 30 * 40 / 50 % 60");
    lexer.lex();
  }
}