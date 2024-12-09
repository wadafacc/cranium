use std::collections::HashMap;

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

const lookup: [(char, &str); 12] = [
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
  ('}', "*ptr = copy;"),
];
