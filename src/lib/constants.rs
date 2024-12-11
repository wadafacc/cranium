pub const HEADER: &str = "
#include <stdio.h> \n
main() { \n
char box[30000], *ptr=box, copy; \n
";

pub const FOOTER: &str = "
\n 
exit(); \n
}";

pub enum Operators {
  IncrCell,
  DecrCell,
  IncrPtr,
  DecrPtr,
  PutChar,
  GetChar,
  StartLoop,
  EndLoop,
  
  // cranium
  ZeroCell,
  InvCell,
  Copy,
  Paste
}

pub const OPERATORS: [(char, &str); 12] = [
  ('-', "--(*ptr);"),
  ('+', "++(*ptr);"),
  ('>', "++ptr;"),
  ('<', "--ptr;"),
  ('.', "putchar(*ptr);"),
  (',', "*ptr = getchar();"),
  ('[', "while (*ptr) {"),
  (']', "}"),
  
  ('_', "*ptr = 0;"),
  ('!', "*ptr *= -1;"),
  ('{', "copy = *ptr;"),
  ('}', "*ptr = copy;")
];


