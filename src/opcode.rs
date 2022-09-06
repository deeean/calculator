#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
  Constant(usize),
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Equal,
  Not,
  Greater,
  Less,
  And,
  Or,
  Call(u8),
  Return,
}