use std::{fs::File, io::Write};

use crate::lib::constants::{ABOUT, DEFAULT_FILENAME, DEFAULT_LEN, FOOTER, HEADER};

use super::{lexer::Lexer, mapper::Mapper};


#[derive(Debug, Clone)]
pub struct Options {
  pub group_args: bool, // group applicable arguments together -> +ptr +ptr => ptr += 2
  pub len: Option<usize>,
  pub filename: Option<String>,
  // pub _run: bool, // run after "compilation"
  // pub _with_tui: bool // attach a tui that shows each cell value for debug
}


pub struct Compiler {
  opts: Options,
  lexer: Lexer,
  mapper: Mapper
}


impl Compiler {
    pub fn new(input: String, opts: Options) -> Self {
      Compiler { opts: opts.clone(), lexer: Lexer::new(input, opts.group_args), mapper: Mapper::new()}
    }

    pub fn comtranspile(&self) -> bool {
      let tokens = self.lexer.map();
      let map = self.mapper.map(tokens);

      if let Ok(mut f) = File::create(self.opts.filename.clone().unwrap_or(DEFAULT_FILENAME.to_string())) {
        _ = f.write_all(ABOUT.as_bytes());  // about
        _ = f.write_all(Mapper::format(HEADER.to_string(),  self.opts.len.unwrap_or(DEFAULT_LEN)).as_bytes());  // about


        for str in map {
         _ = f.write_all(format!("{}\n", str).as_bytes());
        }

        _ = f.write_all(FOOTER.as_bytes());
      }

      true
    }
}