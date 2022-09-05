use crate::ast::BinaryOperator;
use crate::bytecode::Bytecode;
use crate::object::Object;
use crate::opcode::Opcode;

#[derive(Debug)]
pub struct VM {
  ip: usize,
  stack: Vec<Object>,
}

impl Default for VM {
  fn default() -> Self {
    let mut vm = Self {
      ip: 0,
      stack: Vec::new()
    };

    vm.stack.reserve(256);
    vm
  }
}

impl VM {
  fn pop(&mut self) -> Object {
    self.stack.pop().expect("stack underflow")
  }

  fn binary_op(&mut self, left: Object, right: Object, op: BinaryOperator) -> Option<Object> {
    match op {
      BinaryOperator::Add => {
        match (left, right) {
          (Object::Number(left), Object::Number(right)) => Some(Object::Number(left + right)),
          _ => None,
        }
      }
      BinaryOperator::Subtract => {
        match (left, right) {
          (Object::Number(left), Object::Number(right)) => Some(Object::Number(left - right)),
          _ => None,
        }
      }
      BinaryOperator::Multiply => {
        match (left, right) {
          (Object::Number(left), Object::Number(right)) => Some(Object::Number(left * right)),
          _ => None,
        }
      }
      BinaryOperator::Divide => {
        match (left, right) {
          (Object::Number(left), Object::Number(right)) => Some(Object::Number(left / right)),
          _ => None,
        }
      }
      BinaryOperator::Modulo => {
        match (left, right) {
          (Object::Number(left), Object::Number(right)) => Some(Object::Number(left % right)),
          _ => None,
        }
      }
    }
  }

  pub fn run(&mut self, bytecode: Bytecode) -> Option<Object> {
    loop {
      let opcode = bytecode.codes[self.ip];

      match opcode {
        Opcode::Constant(constant) => {
          let constant = bytecode.constants[constant];
          self.stack.push(constant);
        }
        Opcode::Add |
        Opcode::Subtract |
        Opcode::Multiply|
        Opcode::Divide |
        Opcode::Modulo => {
          let right = self.pop();
          let left = self.pop();
          let op = match opcode {
            Opcode::Add => BinaryOperator::Add,
            Opcode::Subtract => BinaryOperator::Subtract,
            Opcode::Multiply => BinaryOperator::Multiply,
            Opcode::Divide => BinaryOperator::Divide,
            Opcode::Modulo => BinaryOperator::Modulo,
            _ => unreachable!()
          };

          let result = match self.binary_op(left, right, op) {
            Some(result) => result,
            None => {
              println!("invalid operation");
              break;
            }
          };

          self.stack.push(result);
        }
        Opcode::Return => {
          return Some(self.pop())
        }
      }

      self.ip += 1;
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use crate::compiler::Compiler;
  use crate::lexer::Lexer;
  use crate::object::Object;
  use crate::parser::Parser;
  use crate::vm::VM;

  #[test]
  fn execute() {
    let testcases = vec![
      (
        "953.55878 - 363.28548 / 337.20 + 964.2119",
        Object::Number(1916.693320925267)
      ),
      (
        "981.187 + 920.748 + 725.14 / 480 + 23.374 + 907.4136 / 139 - 738.1 * 601.3312",
        Object::Number(-441909.210856271)
      ),
      (
        "318.302 - 262 + 779.6 + 596 + 158 / 951.78858 / 310.6",
        Object::Number(1431.9025344599245)
      ),
      (
        "225.19 + 313.495 - 489.5 - 707 - 249.9 % 726 - 920.3 / 300.949 - 906.47581 / 984.2",
        Object::Number(-911.6940212680385)
      ),
      (
        "337.56128 * 117.1258 - 319.95 * 463.42 * 340 - 137.6581 / 740.1 / 335.21 / 954.13 - 251.6826",
        Object::Number(-50372932.40763155)
      ),
      (
        "952.66913 / 205 * 405.9887 / 379.79",
        Object::Number(4.967737647298423)
      ),
      (
        "880.9 * 783.89608 - 880 % 484.0852 - 40.4498",
        Object::Number(690097.692272)
      ),
      (
        "154 + 174.7825 % 984.322 + 328.91390 % 575",
        Object::Number(657.6964)
      ),
      (
        "602.552 % 245.1 * 979.83 + 90.39 % 666.62054 / 386.0230 % 37.3 + 672.7 / 94.525",
        Object::Number(110093.21095284277)
      ),
      (
        "360.82 * 234.9080 / 841.8605 / 854.4 - 969.23 - 473 - 726.820 + 764.9 * 832.73722",
        Object::Number(634791.7674164542)
      ),
      (
        "609.436 / 299.82 - 706.56 / 601 - 791.69 + 684.384 * 120.21 + 506.0 * 143 - 527 % 803.143",
        Object::Number(153309.9676723381)
      ),
      (
        "157 % 494.2 / 244 - 833.7377 % 99.2804 + 554.944 * 131 % 331.2 % 380.481 * 909.70874 * 129.1784",
        Object::Number(19373907.86446046)
      ),
      (
        "582.33 * 730.8 - 449.9284 - 295.534 - 242 - 662.11 % 306 * 626 / 296",
        Object::Number(424473.3257216217)
      ),
      (
        "390.83556 - 371.1 - 30.6 / 584.7 * 869.77425",
        Object::Number(-25.783667039507478)
      ),
      (
        "232.9056 % 732.37134 / 32 * 557.6",
        Object::Number(4058.38008)
      ),
      (
        "432.363 - 352.9 - 594 - 29.2 - 266.276 * 782.33291 + 862.83 % 566.47 - 933.19600 % 214.98",
        Object::Number(-208637.13094316)
      ),
      (
        "27.048 + 422.81241",
        Object::Number(449.86041)
      ),
      (
        "276.2904 - 851.16 % 699.2470 % 784.760 / 227.01 + 565.0 - 36.02 % 727.11292 + 544.6576",
        Object::Number(1349.258809215453)
      ),
      (
        "548 + 176.413 * 963.6 + 375.91737 + 707.5 + 943.52706 * 995 * 343.74 % 725.033 - 344.52928 * 610.13",
        Object::Number(-37896.0220584262)
      ),
      (
        "29.6 * 195.03 + 900.224 - 842.7289 % 843.41432 * 596.1034 - 44.160",
        Object::Number(-495724.6105682599)
      )
    ];

    for (input, expected) in testcases {
      let mut lexer = Lexer::new(input);
      let mut parser = Parser::new(lexer.lex());
      let mut compiler = Compiler::new();
      let mut bytecode = compiler.compile(&parser.parse());
      let mut vm = VM::default();
      let result = vm.run(bytecode).unwrap();

      assert_eq!(result, expected);
    }
  }
}