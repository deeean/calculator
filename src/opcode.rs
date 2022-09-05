#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
  Constant(usize),
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Return,
}