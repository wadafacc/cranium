use crate::lib::constants::*;
/*
--- SYNTAX ---
USUAL BRAINFUCK SYNTAX:
+ -> increment pointer value
- -> decrement pointer value

> -> move pointer to the next cell
< -> move pointer to the previous cell

[ -> start a loop on the current counter; decrement by one every loop until 0
] -> close the loop

. -> output cell value to the console
, -> read a single char

CRANIUM SPECIFIC ADDITONS:
_ -> set value of cell to 0
! -> invert value of cell
{ -> copy value from cell into "copy" slot
} -> move value from "copy" slot into selected cell
*/

#[derive(Debug)]
pub struct Lexer {
  input: String,
  opts: Options
}


#[derive(Debug)]
pub struct Options {
  group_args: bool, // group applicable arguments together -> +ptr +ptr => ptr += 2
  run: bool, // run after "compilation"
  r#override: bool,
  with_tui: bool // attach a tui that shows each cell value for debug
}



#[derive(Debug, PartialEq)]
pub struct Token {
  char: char,
  token: Operator,
  n: usize
}

impl Lexer {
  pub fn new(input: String, opts: Options) -> Lexer {
    // Cleanup input
    let input = input
    .replace("\n", "")
    .replace("\r", "")
    .replace(" ", "");

    Lexer {
      input,
      opts,
    }
  }

  pub fn map(&self) -> Vec<Token> {
    let mut out = Vec::new();
    for c in self.input.chars() {
      if let Some(t) = self.lex(c) {
        if t == *out.last().unwrap_or(&None) {

        }
        
      } 
    }
    out
  } 

  fn lex(&self, c: char) -> Option<Token> {
    if let Some((char, token)) = OPERATORS.into_iter().find(|i|i.0 == c) {
      return Some(Token { char, token, n: 1 });
    }
    None
  }  
}