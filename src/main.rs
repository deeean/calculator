use calculator::compiler::Compiler;
use calculator::lexer::Lexer;
use calculator::parser::Parser;
use calculator::vm::VM;

fn main() -> Result<(), std::io::Error> {
  loop {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    if buffer.trim().is_empty() {
      continue;
    }

    let mut lexer = Lexer::new(&buffer);
    let tokens = lexer.lex();
    let program = Parser::new(tokens).parse();
    let bytecode = Compiler::new().compile(&program);
    let mut vm = VM::default();

    if let Some(res) = vm.run(bytecode) {
      println!("{}", res);
    }
  }

  Ok(())
}
