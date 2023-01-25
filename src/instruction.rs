use std::collections::HashMap;

use tut_language::{Token, TokenType};

pub fn evaluate(instructions: Vec<Instruction>) {
  let stack: &mut Vec<StackItem> = &mut Vec::new();
  let mut variables: HashMap<String, i64> = HashMap::new();

  for instruction in instructions {
    match instruction {
      Instruction::PushId(i) => {
        stack.push(StackItem::Id(i))
      },
      Instruction::PushInteger(i) => {
        stack.push(StackItem::Integer(i))
      },
      Instruction::Assign => {
        // assume right-hand is an integer
        let value = stack.pop().unwrap().as_integer(&variables);
        // assume left-hand is an id
        let name = stack.pop().unwrap().as_id();
      
        variables.insert(name, value);
      }, 
      Instruction::Add => {
        let right = stack.pop().unwrap().as_integer(&variables);
        let left = stack.pop().unwrap().as_integer(&variables);
      
        stack.push(StackItem::Integer(left + right));
      },
      Instruction::Subtract => {
        let right = stack.pop().unwrap().as_integer(&variables);
        let left = stack.pop().unwrap().as_integer(&variables);
      
        stack.push(StackItem::Integer(left - right));
      },
      Instruction::Out => {
        println!("{}", stack.pop().unwrap().as_integer(&variables));
      },
    }
  }
}

#[derive(Debug)]
enum StackItem {
  Id(String),
  Integer(i64),
}

impl StackItem {
  pub fn as_integer(
    self, variables: &HashMap<String, i64>
  ) -> i64 {
    match self {
      Self::Id(s) => variables.get(&s).unwrap().clone(),
      Self::Integer(i) => i,
    }
  }
  pub fn as_id(self) -> String {
    match self {
  	Self::Id(s) => s,
  	_ => panic!("expected Id, got {:?}", self),
	    }
  }
}


#[derive(Debug, Clone)]
pub enum Instruction {
  PushId(String),
  PushInteger(i64),
  Assign,
  Add,
  Subtract,
  Out,
}

impl Instruction {
  pub fn new(token: &Token) -> Self {
    match token.token_type {
      TokenType::Id => {
        if token.lexeme == "out" {
          Self::Out
        } else {
          Self::PushId(token.lexeme.clone())
        }
      },
      TokenType::Integer => {
        Self::PushInteger(token.lexeme.parse().unwrap())
      },
      TokenType::Operator => match token.lexeme.as_str() {
        "+" => Self::Add,
        "-" => Self::Subtract,
        "=" => Self::Assign,
        _ => panic!("invalid operator {:?}!", token.lexeme),
      },      
    }
  }
}
