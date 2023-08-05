use rlox::{
  chunk::{Chunk, OpCode},
  debug::Debug,
  utils::Init,
  vm::VM,
};

fn manual_demo() {
  let mut vm = VM::init();
  let mut chunk = Chunk::init();
  let constant = chunk.add_constant(1.2);
  chunk.write_chunk(OpCode::CONSTANT.into(), 123);
  chunk.write_chunk(constant as u8, 123);
  chunk.write_chunk(OpCode::NEGATE.into(), 123);
  let constant = chunk.add_constant(2.3);
  chunk.write_chunk(OpCode::CONSTANT.into(), 123);
  chunk.write_chunk(constant as u8, 123);
  chunk.write_chunk(OpCode::ADD.into(), 123);
  chunk.write_chunk(OpCode::RETURN.into(), 123);
  chunk.disassemble("Test Chunk");
  vm.interpret(&mut chunk).unwrap();
  vm.free();
}

pub fn main() {
  manual_demo();
}
