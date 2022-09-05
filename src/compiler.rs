use std::rc::Rc;
use crate::ast::{BinaryOperator, Expr, Program, Stmt};
use crate::bytecode::Bytecode;
use crate::object::Object;
use crate::opcode::Opcode;

#[derive(Debug)]
pub struct Compiler {
  bytecode: Bytecode,
}

impl Compiler {
  pub fn new() -> Self {
    Self {
      bytecode: Bytecode::new(),
    }
  }

  fn emit(&mut self, opcode: Opcode) -> usize {
    self.bytecode.codes.push(opcode);
    self.bytecode.codes.len() - 1
  }

  pub fn add_constant(&mut self, constant: Object) -> usize {
    self.bytecode.constants.push(constant);
    self.bytecode.constants.len() - 1
  }

  fn compile_expr(&mut self, expr: &Expr) {
    match expr {
      Expr::Number(n) => {
        let constant = self.add_constant(Object::Number(*n));
        self.emit(Opcode::Constant(constant));
      }
      Expr::BinaryOp(left, op, right) => {
        self.compile_expr(left);
        self.compile_expr(right);

        match op {
          BinaryOperator::Add => self.emit(Opcode::Add),
          BinaryOperator::Subtract => self.emit(Opcode::Subtract),
          BinaryOperator::Multiply => self.emit(Opcode::Multiply),
          BinaryOperator::Divide => self.emit(Opcode::Divide),
          BinaryOperator::Modulo => self.emit(Opcode::Modulo),
        };
      }
    }
  }

  fn compile_stmt(&mut self, stmt: &Stmt) {
    match stmt {
      Stmt::Expr(expr) => {
        self.compile_expr(expr);
      }
    }
  }

  pub fn compile(&mut self, program: &Program) -> Bytecode {
    for stmt in program {
      self.compile_stmt(stmt);
    }

    self.emit(Opcode::Return);

    self.bytecode.clone()
  }
}

#[cfg(test)]
mod tests {
  use crate::compiler::Compiler;
  use crate::lexer::Lexer;
  use crate::parser::Parser;

  #[test]
  fn compile() {
    let mut lexer = Lexer::new("10 + 20 - 30 * 40 / 50 % 60");
    let mut parser = Parser::new(lexer.lex());
    let mut compiler = Compiler::new();
    let mut bytecode = compiler.compile(&parser.parse());

    println!("{:#?}", bytecode);
  }
}