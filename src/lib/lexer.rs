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
  input: String
}

#[derive(Debug)]
pub struct Token {
  char: char,
  n: usize, // how many times it's been repeated
  out: String
}

impl Lexer {
  pub fn init(input: String) -> Lexer {
    let input = input
    .replace("\n", "")
    .replace("\r", "")
    .replace(" ", "");

    Lexer {
      input
    }
  }

  pub fn map(&self) -> Vec<Token> {
    if let op = OPERATORS.iter().find(|i|i.0 == c) {
      
    }

    todo!()
  } 

  fn lex(&self, c: char, n: usize) -> Token {
    
    if n > 1 {

    }


    todo!()
  }
}