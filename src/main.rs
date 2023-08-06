use rlox::{utils, vm::InterpretError, vm::VM};
use std::io::{self, Write};
use std::process::exit;

pub fn main() {
  let argv = utils::args();
  let argc = argv.len();
  if argc > 2 {
    eprintln!("Usage: rlox [path]");
    exit(64);
  }

  let mut vm = VM::init();
  if argc == 1 {
    repl(&mut vm).unwrap();
  } else if argc == 2 {
    run_file(&mut vm, argv[1].to_owned()).unwrap();
  }
  vm.free();
}

/// Run the REPL.
fn repl(vm: &mut VM) -> Result<(), InterpretError> {
  println!("Welcome to lox's REPL!");
  println!("Press <Ctrl> + <C> to exit.");
  loop {
    print!("|> ");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    if let Err(e) = vm.interpret(line) {
      eprintln!("{:?}", e);
    }
  }
}

/// Run the given file.
fn run_file(vm: &mut VM, path: String) -> Result<(), InterpretError> {
  vm.interpret_file(path)
}
