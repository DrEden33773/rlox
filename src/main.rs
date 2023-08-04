use std::{
  cmp::Ordering::{Equal, Greater, Less},
  env::args,
  fs::File,
  io::{self, BufReader, Read},
  process::exit,
};

pub fn main() {
  let args = args().collect::<Vec<_>>();
  let (_root, args) = args.split_first().unwrap();
  match args.len().cmp(&1) {
    Greater => {
      eprintln!("Usage: rlox [script]");
      exit(64);
    }
    Equal => run_file(&args[0]).unwrap(),
    Less => run_prompt(&[]),
  }
}

fn run_file(path: &str) -> io::Result<()> {
  let file = File::open(path)?;
  let mut buffer = Vec::new();
  BufReader::new(file).read_to_end(&mut buffer)?;
  run_prompt(&buffer);
  Ok(())
}

fn run_prompt(src: &[u8]) {
  match src.is_empty() {
    true => {
      println!("Unimplemented");
    }
    false => {
      let src = String::from_utf8_lossy(src);
      src.lines().for_each(|line| {
        run(line);
      });
    }
  }
}

fn run(_src: &str) {}
