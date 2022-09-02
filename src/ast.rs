#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
  Number(f64),
  BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
  Expr(Expr),
}

pub type Program = Vec<Stmt>;