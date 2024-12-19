use crate::ast::Statement;
use crate::parser::{parse, ParseError};
use std::cell::RefCell;
use std::io::{self, Write};

// pub enum Error {
//   SyntaxError(ParseError),
//   RuntimeError(String),
// }

const CAPACITY: usize = 1024;

// 定义 IO trait
pub trait BrainfuckIO {
  fn read_char(&mut self) -> Option<u8>;
  fn write_char(&mut self, byte: u8);
}

// 标准 IO 实现
pub struct StandardIO;

impl BrainfuckIO for StandardIO {
  fn read_char(&mut self) -> Option<u8> {
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_ok() {
      s.chars().next().map(|c| c as u8)
    } else {
      None
    }
  }

  fn write_char(&mut self, byte: u8) {
    print!("{}", byte as char);
    io::stdout().flush().unwrap();
  }
}

// 测试用 IO 实现
#[cfg(test)]
pub struct TestIO {
  input: Vec<u8>,
  output: Vec<u8>,
  input_pos: usize,
}

#[cfg(test)]
impl TestIO {
  pub fn new(input: Vec<u8>) -> Self {
    TestIO {
      input,
      output: Vec::new(),
      input_pos: 0,
    }
  }

  pub fn get_output(&self) -> String {
    String::from_utf8_lossy(&self.output).into_owned()
  }
}

#[cfg(test)]
impl BrainfuckIO for TestIO {
  fn read_char(&mut self) -> Option<u8> {
    if self.input_pos < self.input.len() {
      let ch = self.input[self.input_pos];
      self.input_pos += 1;
      Some(ch)
    } else {
      None
    }
  }

  fn write_char(&mut self, byte: u8) {
    self.output.push(byte);
  }
}

// 修改 Runtime 结构体
pub(crate) struct Runtime<T: BrainfuckIO> {
  mem: [u8; CAPACITY],
  ptr: usize,
  output: Vec<u8>,
  io: T,
}

impl<T: BrainfuckIO> Runtime<T> {
  pub fn new(mem: [u8; CAPACITY], ptr: usize, io: T) -> Runtime<T> {
    Runtime {
      mem,
      ptr,
      output: Vec::new(),
      io,
    }
  }

  #[cfg(test)]
  pub fn get_memory_at(&self, index: usize) -> Option<u8> {
    if index < CAPACITY {
      Some(self.mem[index])
    } else {
      None
    }
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
            if let Some(byte) = self.io.read_char() {
              self.mem[self.ptr] = byte;
            }
          }
          Statement::OutputStmt => {
            self.output.push(self.mem[self.ptr]);
            self.io.write_char(self.mem[self.ptr]);
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

// 修改运行函数
pub fn run_code(source: &str) {
  let mut runtime = Runtime::new([0u8; CAPACITY], 0, StandardIO);
  match parse(source) {
    Ok(stmts) => runtime.execute_stmts(&stmts.body, true),
    Err(ParseError::LexerError(e)) => eprintln!("词法分析错误: {:?}", e),
    Err(e) => eprintln!("语法分析错误: {:?}", e),
  }
}

// 修改测试函数
#[cfg(test)]
pub(crate) fn run_code_test(source: &str, input: Vec<u8>) -> Option<Runtime<TestIO>> {
  let mut runtime = Runtime::new([0u8; CAPACITY], 0, TestIO::new(input));
  let ast = parse(source);
  if let Result::Ok(stmts) = ast {
    runtime.execute_stmts(&stmts.body, true);
    Some(runtime)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_input_operation() {
    let source = ",."; // 读取一个字符并输出
    let input = vec![b'A'];
    let runtime = run_code_test(source, input).unwrap();
    assert_eq!(runtime.io.get_output(), "A");
    assert_eq!(runtime.get_memory_at(0), Some(b'A'));
  }

  #[test]
  fn test_basic_increment() {
    let source = "++++.";
    let runtime = run_code_test(source, vec![]).unwrap();
    assert_eq!(runtime.io.get_output(), "\u{4}");
    assert_eq!(runtime.get_memory_at(0), Some(4));
  }

  #[test]
  fn test_hello_world() {
    // 一个简单的 Hello World 程序
    let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let runtime = run_code_test(source, vec![]).unwrap();
    assert_eq!(runtime.io.get_output(), "Hello World!\n");
  }

  #[test]
  fn test_memory_operations() {
    let source = "+++>++>+"; // 在前三个单元格中分别设置值 3,2,1
    let runtime = run_code_test(source, vec![]).unwrap();
    assert_eq!(runtime.get_memory_at(0), Some(3));
    assert_eq!(runtime.get_memory_at(1), Some(2));
    assert_eq!(runtime.get_memory_at(2), Some(1));
  }

  #[test]
  fn test_simple_loop() {
    let source = "++[>+++<-]>"; // 将会在第二个单元格中产生 6
    let runtime = run_code_test(source, vec![]).unwrap();
    assert_eq!(runtime.get_memory_at(1), Some(6));
    assert_eq!(runtime.get_memory_at(0), Some(0));
  }

  #[test]
  fn test_out_of_bounds_memory_access() {
    let source = ">"; // 移动指针到位置 1
    let runtime = run_code_test(source, vec![]).unwrap();
    assert_eq!(runtime.get_memory_at(CAPACITY), None);
  }
}
