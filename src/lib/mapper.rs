use super::{constants::MAPPINGS, constants::Operator::*, lexer::Token};

pub struct Mapper { }


impl Mapper {
  pub fn new() -> Self {
    Mapper { }
  }

  pub fn map(&self, map: Vec<Token>) -> Vec<String> {
    let mut out = Vec::new();

    for t in map {
      if let Some(mut m) = Self::lookup(t) {
        if [IncrCell, DecrCell, IncrPtr, DecrPtr].contains(&t.token) {
          m = Self::format(m, t.n);
        }
        out.push(m);
      }
    }

    out
  }


  fn lookup(t: Token) -> Option<String> {
    if let Some((_, token)) = MAPPINGS.into_iter().find(|i|i.0 == t.token) {
      return Some(token.to_string());
    }
    None
  }


  pub fn format(str: String, val: usize) -> String {
    str.replace("{}", &val.to_string())
  }
}