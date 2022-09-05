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

  fn is_at_end(&self) -> bool {
    self.curr >= self.input.len()
  }

  fn peek(&self) -> u8 {
    if self.is_at_end() {
      b'\0'
    } else {
      self.input.as_bytes()[self.curr]
    }
  }

  fn advance(&mut self) -> u8 {
    let ch = self.peek();
    self.curr = self.next;
    self.next += 1;
    ch
  }

  fn skip_whitespace(&mut self) {
    loop {
      match self.peek() {
        b' ' | b'\n' | b'\r' | b'\t' => {
          self.advance();
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
          self.advance();
        }
        _ => {
          break;
        }
      }
    }

    if self.peek() == b'.' {
      self.advance();

      loop {
        match self.peek() {
          b'0'..=b'9' => {
            self.advance();
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

  fn read_identifier(&mut self) -> Token<'a> {
    let start = self.curr;

    loop {
      match self.peek() {
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => {
          self.advance();
        }
        _ => {
          break;
        }
      }
    }

    let slice = &self.input[start..self.curr];

    Token::new(TokenKind::Identifier, slice)
  }

  fn token(&mut self) -> Token<'a> {
    self.skip_whitespace();

    let curr = self.curr;

    let kind = match self.peek() {
      b'0'..=b'9' => {
        return self.read_number();
      }
      b',' => {
        TokenKind::Comma
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
      b'a' ..= b'z' | b'A' ..= b'Z' => {
        return self.read_identifier();
      }
      b'\0' => {
        return Token::new(TokenKind::Eof, "");
      }
      _ => {
        panic!("Unexpected character: {}", self.peek() as char)
      },
    };

    let slice = &self.input[curr..self.next];
    self.advance();
    Token::new(kind, slice)
  }

  pub fn lex(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();

    loop {
      let token = self.token();
      tokens.push(token);

      if let Some(last_token) = tokens.last() {
        if last_token.kind == TokenKind::Eof {
          break;
        }
      }
    }

    return tokens;
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn lexer() {
    let mut lexer = super::Lexer::new("(3.141592 + 20) - 30 * 40 / 50 % 60");
    let tokens = lexer.lex();

    println!("{:#?}", tokens);
  }
}