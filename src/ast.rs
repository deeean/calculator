#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
  Negative,
  Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Equal,
  NotEqual,
  LessThan,
  GreaterThan,
  LessThanOrEqual,
  GreaterThanOrEqual,
  And,
  Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Number(f64),
  Boolean(bool),
  Identifier(String),
  UnaryOp(UnaryOperator, Box<Expr>),
  BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
  Call(Box<Expr>, Box<Vec<Expr>>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
  Expr(Expr),
}

pub type Program = Vec<Stmt>;