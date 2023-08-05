#[test]
fn manual_demo() {
  use rlox::{
    chunk::{Chunk, OpCode},
    debug::Debug,
    utils::Init,
    vm::VM,
  };

  let mut vm = VM::init();
  let mut chunk = Chunk::init();

  // 1.2
  let constant = chunk.add_constant(1.2);
  chunk.write_chunk(OpCode::CONSTANT.into(), 123);
  chunk.write_chunk(constant as u8, 123);
  // 2.3
  let constant = chunk.add_constant(2.3);
  chunk.write_chunk(OpCode::CONSTANT.into(), 123);
  chunk.write_chunk(constant as u8, 123);
  // +
  chunk.write_chunk(OpCode::ADD.into(), 123);
  // 5.6
  let constant = chunk.add_constant(5.6);
  chunk.write_chunk(OpCode::CONSTANT.into(), 123);
  chunk.write_chunk(constant as u8, 123);
  // /
  chunk.write_chunk(OpCode::DIVIDE.into(), 123);
  // -
  chunk.write_chunk(OpCode::NEGATE.into(), 123);
  // return
  chunk.write_chunk(OpCode::RETURN.into(), 123);

  chunk.disassemble("Test Chunk");
  vm.interpret(&mut chunk).unwrap();
  vm.free();
}
