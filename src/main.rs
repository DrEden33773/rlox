use std::env::args;

use rlox::{
  chunk::{Chunk, OpCode},
  debug::Debug,
  utils::Init,
};

/// Get the command line arguments
///
/// We have to rewrite this, as [`std::env::args`]'s first argument
/// is always the path to the executable
fn get_args() -> Vec<String> {
  let args = args().collect::<Vec<_>>();
  let (_, args) = args.split_first().unwrap();
  args.to_vec()
}

pub fn main() {
  let _args = get_args();
  let mut chunk = Chunk::init();
  chunk.write(OpCode::RETURN as u8);
  chunk.disassemble("Test Chunk");
}
