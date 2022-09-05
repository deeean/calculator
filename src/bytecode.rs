use crate::opcode::Opcode;
use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Bytecode {
  pub codes: Vec<Opcode>,
  pub constants: Vec<Object>,
}

impl Bytecode {
  pub fn new() -> Self {
    Self {
      codes: Vec::new(),
      constants: Vec::new(),
    }
  }
}