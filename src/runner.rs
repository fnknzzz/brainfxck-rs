use crate::ast::Statement;
use crate::parser::parse;
use std::cell::RefCell;
use std::io::stdin;

// pub enum Error {
//   SyntaxError(ParseError),
//   RuntimeError(String),
// }

const CAPACITY: usize = 1024;

struct Runtime {
  mem: [u8; CAPACITY],
  ptr: usize,
}

impl Runtime {
  pub fn new(mem: [u8; CAPACITY], ptr: usize) -> Runtime {
    Runtime { mem, ptr }
  }
  pub fn execute_stmts(&mut self, stmts: &RefCell<Vec<Statement>>, is_top: bool) {
    loop {
      if !is_top && self.mem[self.ptr] == 0 {
        break;
      }
      for stmt in stmts.borrow().iter() {
        match stmt {
          Statement::DecPtrStmt => self.ptr = self.ptr - 1,
          Statement::IncPtrStmt => self.ptr = self.ptr + 1,
          Statement::DecValStmt => self.mem[self.ptr] = self.mem[self.ptr] - 1,
          Statement::IncValStmt => self.mem[self.ptr] = self.mem[self.ptr] + 1,
          Statement::InputStmt => {
            let mut s = String::new();
            stdin().read_line(&mut s).expect("please input");
            if let Some(char) = s.chars().next() {
              self.mem[self.ptr] = char as u8;
            }
          }
          Statement::OutputStmt => {
            println!("{}", self.mem[self.ptr] as char);
          }
          Statement::BracketsStmt(stmt) => {
            self.execute_stmts(&stmt, false);
          }
        }
      }
      if is_top {
        break;
      }
    }
  }
}

pub fn run_code<'a>(source: &'a str) {
  let mut runtime = Runtime::new([0u8; CAPACITY], 0);
  let ast = parse(source);
  if let Result::Ok(stmts) = ast {
    runtime.execute_stmts(&stmts.body, true);
  }
}
