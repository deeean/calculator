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