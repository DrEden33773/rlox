use std::env::args;

use rlox::{
  chunk::{Chunk, OpCode},
  debug::Debug,
  utils::Init,
  vm::VM,
};

/// Get the command line arguments
///
/// We have to rewrite this, as [`std::env::args`]'s first argument
/// is always the path to the executable
#[allow(dead_code)]
fn get_args() -> Vec<String> {
  let args = args().collect::<Vec<_>>();
  let (_, args) = args.split_first().unwrap();
  args.to_vec()
}

fn manual_demo() {
  let mut vm = VM::init();
  let mut chunk = Chunk::init();
  let constant = chunk.add_constant(1.2);
  chunk.write_chunk(OpCode::CONSTANT.into(), 123);
  chunk.write_chunk(constant as u8, 123);
  chunk.write_chunk(OpCode::RETURN.into(), 123);
  chunk.disassemble("Test Chunk");
  vm.interpret(&mut chunk).unwrap();
  vm.free();
}

pub fn main() {
  manual_demo();
}
