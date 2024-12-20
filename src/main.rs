#[allow(special_module_name)]

use std::fs::read_to_string;
use lib::compiler::{Compiler, Options};

mod lib;

fn main() {
  let input = read_to_string("./test.bf").expect("no file present");

  let comp = Compiler::new(input, Options {group_args: true, run: false,len: Some(42069), filename:Some(String::from("example.c")), with_tui: false});

  comp.comtranspile();
}
