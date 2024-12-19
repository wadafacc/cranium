use crate::lib::constants::Operator::*;

pub const HEADER: &str = "
#include <stdio.h> \n
main() { \n
char box[{}], *ptr=box, copy; \n
";

pub const FOOTER: &str = "
\n 
exit(); \n
}";

#[derive(Debug, PartialEq, Default)]
pub enum Operator {
  IncrCell,
  DecrCell,
  IncrPtr,
  DecrPtr,
  Print,
  Read,
  StartLoop,
  EndLoop,
  
  // cranium
  ZeroCell,
  InvCell,
  Copy,
  Paste,

  #[default]
  Empty
}

pub const OPERATORS: [(char, Operator); 12] = [
  ('+', IncrCell),
  ('-', DecrCell),
  ('>', IncrPtr),
  ('<', DecrPtr),
  ('.', Print),
  (',', Read),
  ('[', StartLoop),
  (']', EndLoop),
  
  ('_', ZeroCell),
  ('!', InvCell),
  ('{', Copy),
  ('}', Paste)
];

pub const MAPPINGS: [(Operator, &str); 12] = [
  (IncrCell, "++(*ptr);"),
  (DecrCell, "--(*ptr);"),
  (IncrPtr, "++ptr;"),
  (DecrPtr, "--ptr;"),
  (Print, "putchar(*ptr);"),
  (Read, "*ptr = getchar();"),
  (StartLoop, "while (*ptr) {"),
  (EndLoop, "}"),
  
  (ZeroCell, "*ptr = 0;"),
  (InvCell, "*ptr *= -1;"),
  (Copy, "copy = *ptr;"),
  (Paste, "*ptr = copy;")
];

