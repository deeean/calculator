#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Number(f64),
  Identifier(String),

  BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
  Call(Box<Expr>, Box<Vec<Expr>>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
  Expr(Expr),
}

pub type Program = Vec<Stmt>;