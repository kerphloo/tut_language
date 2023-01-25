// import Token and TokenType from lib.rs
use tut_language::{Token, TokenType};

const OPERATOR: &str = "-+=";
const INTEGER: &str = "0123456789";
const ID: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum State {
  Searching,
	FoundId,
	FoundInteger,
  FoundOperator,
}

impl State {
	  pub fn new(string: &str) -> Self {
    if ["", " ", "\n", "\r"].contains(&string) {
      return Self::Searching;
    }

    if OPERATOR.contains(string) {
      return Self::FoundOperator;
    } else if INTEGER.contains(string) {
      return Self::FoundInteger;
    } else if ID.contains(string) {
      return Self::FoundId;
    } else {
      panic!("unexpected character {:?}", string);
    }
  }
}

pub struct Tokenizer {
  state: State,
}

impl Tokenizer {
  pub fn new() -> Self {
    return Self {
      state: State::Searching
    };
  }
  pub fn scan(&mut self, source: String, tokens: &mut Vec<Token>) {
    let mut buffer = Vec::new(); // empty vector

    for character in source.split("") { // split at every character
      // get the type of the current character
      let character_type = State::new(character);

      // 1. character type/state mismatch
      if character_type != self.state {
        if buffer.len() != 0 { // if buffer not empty
          tokens.push(Token::new(
            match self.state {
              State::FoundId => TokenType::Id,
              State::FoundInteger => TokenType::Integer,
              State::FoundOperator => TokenType::Operator,
              _ => panic!(
                "can’t push a token of type {:?}",
                self.state,
              ),
            },
            buffer.join(""), // join buffer into one string
          ));
          self.state = character_type;
          buffer.clear();
        }
      }

      // 2. character type matches Searching
      if character_type == State::Searching {
        self.state = character_type;
        continue;
      }

      // 3. scanner state matches Searching
      if self.state == State::Searching {
        self.state = character_type;
      } if self.state != State::Searching {
        buffer.push(character);
      }      
    }
  } 
}
