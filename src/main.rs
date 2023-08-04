use std::env::args;

use rlox::chunk::{Chunk, OpCode};

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
  let args = get_args();
  let mut chunk = Chunk::default();
  chunk.write(OpCode::RETURN as u8);
}
