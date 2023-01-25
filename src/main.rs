mod instruction;
mod tokenizer;

use instruction::{evaluate, Instruction};
use tokenizer::Tokenizer;

// i/o imports
use std::env;
use std::fs::read_to_string;

fn main() {
  // read the first command-line argument as a file path
  let path = &env::args().collect::<Vec<String>>()[1];

  let tokens = &mut Vec::new();
  Tokenizer::new().scan(
    // read that file as a string
    read_to_string(path).unwrap(),
    tokens,
  );

  let mut instructions = Vec::new();
  for token in tokens {
    instructions.push(Instruction::new(token))
  }

  evaluate(instructions);
}
