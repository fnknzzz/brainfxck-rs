use std::cell::RefCell;

pub struct Program {
  pub body: RefCell<Vec<Statement>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
  IncPtrStmt,
  DecPtrStmt,
  IncValStmt,
  DecValStmt,
  OutputStmt,
  InputStmt,
  BracketsStmt(RefCell<Vec<Statement>>),
}
