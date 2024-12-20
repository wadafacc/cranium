use crate::lib::constants::{Operator::*, *};

use super::compiler::Options;
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
  group_args: bool
}


#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Token {
  pub char: char,
  pub token: Operator,
  pub n: usize
}

impl Lexer {
  pub fn new(input: String, group_args: bool) -> Lexer {
    // Cleanup input
    let input = input
    .replace("\n", "")
    .replace("\r", "")
    .replace(" ", "");

    Lexer {
      input,
      group_args,
    }
  }

  pub fn map(&self) -> Vec<Token> {
    let mut out :Vec<Token> = Vec::new();
    for c in self.input.chars() {
      if let Some(mut t) = self.lex(c) {
        // only check previous if its a + or -, < or >
        if self.group_args && (t.char == out.last().unwrap_or(&Token{..Default::default()}).char && [IncrCell, DecrCell, IncrPtr, DecrPtr].contains(&t.token)){
          let last = out.pop().unwrap();
          t.n += last.n;  // incr counter
        }
        out.push(t);    
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