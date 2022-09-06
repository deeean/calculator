use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Object {
  Number(f64),
  Boolean(bool),
}

impl Object {
  pub fn is_truthy(object: &Object) -> bool {
    match object {
      Object::Boolean(b) => *b,
      Object::Number(n) => *n != 0.0,
    }
  }

  pub fn is_falsey(object: &Object) -> bool {
    !Object::is_truthy(object)
  }
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Object::Number(n) => write!(f, "{}", n),
      Object::Boolean(b) => write!(f, "{}", b),
    }
  }
}