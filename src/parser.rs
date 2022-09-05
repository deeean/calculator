use crate::ast::{BinaryOperator, Expr, Program, Stmt};
use crate::token::{Token, TokenKind};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
  None,
  Term,
  Factor,
  Unary,
  Grouping,
  Call,
}

impl Precedence {
  pub fn from(kind: TokenKind) -> Precedence {
    match kind {
      TokenKind::Plus | TokenKind::Minus => Precedence::Term,
      TokenKind::Slash | TokenKind::Star | TokenKind::Percent => Precedence::Factor,
      TokenKind::LeftParen => Precedence::Grouping,
      _ => Precedence::None,
    }
  }
}

#[derive(Debug)]
pub struct Parser<'a> {
  tokens: Vec<Token<'a>>,
  curr: usize,
  next: usize,
}

impl <'a> Parser<'a> {
  pub fn new(tokens: Vec<Token<'a>>) -> Self {
    Self {
      tokens,
      curr: 0,
      next: 1,
    }
  }

  fn is_at_end(&self) -> bool {
    self.curr >= self.tokens.len()
  }

  fn peek(&self) -> Token<'a> {
    if self.is_at_end() {
      self.tokens[self.tokens.len() - 1].clone()
    } else {
      self.tokens[self.curr].clone()
    }
  }

  fn next_peek(&self) -> Token<'a> {
    if self.next >= self.tokens.len() {
      self.tokens[self.tokens.len() - 1].clone()
    } else {
      self.tokens[self.next].clone()
    }
  }

  fn advance(&mut self) -> Token<'a> {
    let token = self.peek();
    self.curr = self.next;
    self.next += 1;
    token.clone()
  }

  fn parse_number_expr(&mut self) -> Option<Expr> {
    match self.peek().kind {
      TokenKind::Number => {
        let value = self.peek().slice.parse::<f64>().unwrap();
        Some(Expr::Number(value))
      }
      _ => None,
    }
  }

  fn parse_binary_op_expr(&mut self, left: Option<Expr>) -> Option<Expr> {
    let left = match left {
      Some(expr) => expr,
      None => return None,
    };

    let op = match self.peek().kind {
      TokenKind::Plus => BinaryOperator::Add,
      TokenKind::Minus => BinaryOperator::Subtract,
      TokenKind::Star => BinaryOperator::Multiply,
      TokenKind::Slash => BinaryOperator::Divide,
      TokenKind::Percent => BinaryOperator::Modulo,
      _ => return None,
    };

    let precedence = Precedence::from(self.peek().kind);

    self.advance();

    match self.parse_expr(precedence) {
      Some(right) => {
        Some(Expr::BinaryOp(Box::new(left), op, Box::new(right)))
      },
      None => None,
    }
  }

  fn parse_grouping_expr(&mut self) -> Option<Expr> {
    self.advance();

    let expr = match self.parse_expr(Precedence::None) {
      Some(expr) => expr,
      None => return None,
    };

    self.advance();

    Some(expr)
  }

  fn parse_expr_list(&mut self, end_token_kind: TokenKind) -> Option<Vec<Expr>> {
    let mut exprs = Vec::new();

    if self.next_peek().kind == end_token_kind {
      self.advance();
      return Some(exprs);
    }

    self.advance();

    match self.parse_expr(Precedence::None) {
      Some(expr) => {
        exprs.push(expr);
      },
      None => return None,
    };

    while self.next_peek().kind == TokenKind::Comma {
      self.advance();
      self.advance();

      match self.parse_expr(Precedence::None) {
        Some(expr) => {
          exprs.push(expr);
        },
        None => return None,
      };
    }

    if self.next_peek().kind != end_token_kind {
      return None;
    }

    self.advance();

    Some(exprs)
  }

  fn parse_call_expr(&mut self, left: Option<Expr>) -> Option<Expr> {
    let left = match left {
      Some(expr) => expr,
      None => return None,
    };

    let args = match self.parse_expr_list(TokenKind::RightParen) {
      Some(args) => args,
      None => return None,
    };

    Some(Expr::Call(Box::new(left), Box::new(args)))
  }

  fn parse_identifier(&mut self) -> Option<String> {
    match self.peek().kind {
      TokenKind::Identifier => Some(self.peek().slice.to_string()),
      _ => None,
    }
  }

  fn parse_identifier_expr(&mut self) -> Option<Expr> {
    match self.parse_identifier() {
      Some(identifier) => Some(Expr::Identifier(identifier)),
      None => None,
    }
  }

  fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
    let mut left = match self.peek().kind {
      TokenKind::Number => self.parse_number_expr(),
      TokenKind::LeftParen => self.parse_grouping_expr(),
      TokenKind::Identifier => self.parse_identifier_expr(),
      _ => return None
    };

    while self.next_peek().kind != TokenKind::Eof && precedence < Precedence::from(self.next_peek().kind) {
      match self.next_peek().kind {
        TokenKind::Plus |
        TokenKind::Minus |
        TokenKind::Star |
        TokenKind::Slash |
        TokenKind::Percent => {
          self.advance();
          left = self.parse_binary_op_expr(left);
        }
        TokenKind::LeftParen => {
          self.advance();
          left = self.parse_call_expr(left);
        }
        _ => return left,
      }
    }

    left
  }

  fn parse_expr_stmt(&mut self) -> Option<Stmt> {
    match self.parse_expr(Precedence::None) {
      Some(expr) => {
        if self.next_peek().kind == TokenKind::Eof {
          self.advance();
        }

        Some(Stmt::Expr(expr))
      },
      None => None
    }
  }

  fn parse_stmt(&mut self) -> Option<Stmt> {
    match self.peek().kind {
      _ => self.parse_expr_stmt()
    }
  }

  pub fn parse(&mut self) -> Program {
    let mut stmts = Program::new();

    while !self.is_at_end() {
      let stmt = self.parse_stmt();
      if let Some(stmt) = stmt {
        stmts.push(stmt);
      }

      self.advance();
    }

    stmts
  }
}

#[cfg(test)]
mod tests {
  use crate::ast::{BinaryOperator, Expr, Stmt};
  use crate::lexer::Lexer;
  use crate::parser::Parser;

  #[test]
  fn parser() {
    let testcases = vec![
      (
        "1 + 2",
        vec![
          Stmt::Expr(
            Expr::BinaryOp(
              Box::new(Expr::Number(1.0)),
              BinaryOperator::Add,
              Box::new(Expr::Number(2.0))
            )
          )
        ]
      ),
      (
        "1 + 2 * 3",
        vec![
          Stmt::Expr(
            Expr::BinaryOp(
              Box::new(Expr::Number(1.0)),
              BinaryOperator::Add,
              Box::new(
                Expr::BinaryOp(
                  Box::new(Expr::Number(2.0)),
                  BinaryOperator::Multiply,
                  Box::new(Expr::Number(3.0))
                )
              )
            )
          )
        ]
      ),
      (
        "(1 + 2) * 3",
        vec![
          Stmt::Expr(
            Expr::BinaryOp(
              Box::new(
                Expr::BinaryOp(
                  Box::new(Expr::Number(1.0)),
                  BinaryOperator::Add,
                  Box::new(Expr::Number(2.0))
                )
              ),
              BinaryOperator::Multiply,
              Box::new(Expr::Number(3.0))
            )
          )
        ]
      ),
      (
        "1 + 2 * 3 + 4",
        vec![
          Stmt::Expr(
            Expr::BinaryOp(
              Box::new(
                Expr::BinaryOp(
                  Box::new(Expr::Number(1.0)),
                  BinaryOperator::Add,
                  Box::new(
                    Expr::BinaryOp(
                      Box::new(Expr::Number(2.0)),
                      BinaryOperator::Multiply,
                      Box::new(Expr::Number(3.0))
                    )
                  )
                )
              ),
              BinaryOperator::Add,
              Box::new(Expr::Number(4.0))
            )
          )
        ]
      ),
      (
        "sin(1)",
        vec![
          Stmt::Expr(
            Expr::Call(
              Box::new(Expr::Identifier("sin".to_string())),
              Box::new(vec![
                Expr::Number(1.0)
              ])
            )
          )
        ]
      )
    ];


    for (input, expected) in testcases {
      let mut lexer = Lexer::new(input);
      let mut parser = Parser::new(lexer.lex());

      println!("{}", input);
      assert_eq!(parser.parse(), expected);
    }
  }
}