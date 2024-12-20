use crate::lib::constants::Operator::*;

pub const DEFAULT_LEN: usize = 30000;
pub const DEFAULT_FILENAME: &str = "out.c";

pub const ABOUT: &str = r"
/*
*      ___           ___           ___           ___                       ___           ___              
*     /\  \         /\  \         /\  \         /\__\          ___        /\__\         /\__\             
*    /::\  \       /::\  \       /::\  \       /::|  |        /\  \      /:/  /        /::|  |            
*   /:/\:\  \     /:/\:\  \     /:/\:\  \     /:|:|  |        \:\  \    /:/  /        /:|:|  |            
*  /:/  \:\  \   /::\~\:\  \   /::\~\:\  \   /:/|:|  |__      /::\__\  /:/  /  ___   /:/|:|__|__          
* /:/__/ \:\__\ /:/\:\ \:\__\ /:/\:\ \:\__\ /:/ |:| /\__\  __/:/\/__/ /:/__/  /\__\ /:/ |::::\__\         
* \:\  \  \/__/ \/_|::\/:/  / \/__\:\/:/  / \/__|:|/:/  / /\/:/  /    \:\  \ /:/  / \/__/~~/:/  /         
*  \:\  \          |:|::/  /       \::/  /      |:/:/  /  \::/__/      \:\  /:/  /        /:/  /          
*   \:\  \         |:|\/__/        /:/  /       |::/  /    \:\__\       \:\/:/  /        /:/  /           
*    \:\__\        |:|  |         /:/  /        /:/  /      \/__/        \::/  /        /:/  /            
*     \/__/         \|__|         \/__/         \/__/                     \/__/         \/__/             
*                                     
*                                        C O M P I L E R 
*                                     ~ made by @wadafacc ~
*
*/
";

pub const HEADER: &str = "
#include <stdio.h> \n
main() {
char box[{}], *ptr=box, copy; \n
";

pub const FOOTER: &str = "
exit();
}";

#[derive(Debug, PartialEq, Default, Clone, Copy)]
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
  (IncrCell, "*ptr += {};"),
  (DecrCell, "*ptr -= {};"),
  (IncrPtr, "ptr += {};"),
  (DecrPtr, "ptr += {};"),
  (Print, "putchar(*ptr);"),
  (Read, "*ptr = getchar();"),
  (StartLoop, "while (*ptr) {"),
  (EndLoop, "}"),
  
  (ZeroCell, "*ptr = 0;"),
  (InvCell, "*ptr *= -1;"),
  (Copy, "copy = *ptr;"),
  (Paste, "*ptr = copy;")
];

