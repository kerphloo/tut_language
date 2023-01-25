#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
}

impl Token {
	  pub fn new(token_type: TokenType, source: String) -> Self {
	    return Self {
	      token_type: token_type,
	      lexeme: source,
	    };
	  }
}

#[derive(Debug)]
pub enum TokenType {
  Id,
  Integer,
  Operator
}
