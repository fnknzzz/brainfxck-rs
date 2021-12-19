use crate::ast::{Program, Statement};
use crate::lexer::{Lexer, Token};
use std::cell::RefCell;

struct State {
  current_stmts: Vec<RefCell<Vec<Statement>>>,
}

macro_rules! add_token {
  ($state: ident, $token: ident) => {
    $state
      .current_stmts
      .last()
      .unwrap()
      .borrow_mut()
      .push(Statement::$token);
  };
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
  UnexceptedTokenRightBracket,
  UnclosedLeftBracket,
}

pub fn parse<'a>(source: &'a str) -> Result<Program, ParseError> {
  let lexer = Lexer::new(source);
  let mut state = State {
    current_stmts: vec![RefCell::new(vec![])],
  };
  for token in lexer.tokens.iter() {
    match token {
      Token::GreaterThan => {
        add_token!(state, IncPtrStmt);
      }
      Token::LessThan => {
        add_token!(state, DecPtrStmt);
      }
      Token::Plus => {
        add_token!(state, IncValStmt);
      }
      Token::Minus => {
        add_token!(state, DecValStmt);
      }
      Token::Dot => {
        add_token!(state, OutputStmt);
      }
      Token::Comma => {
        add_token!(state, InputStmt);
      }
      Token::LeftBrackets => {
        state.current_stmts.push(RefCell::new(vec![]));
      }
      Token::RightBrackes => {
        if state.current_stmts.len() < 2 {
          return Result::Err(ParseError::UnexceptedTokenRightBracket);
        }
        let closed_scope = state.current_stmts.pop().unwrap();
        let mut current_scope = state.current_stmts.last().unwrap().borrow_mut();
        current_scope.push(Statement::BracketsStmt(closed_scope));
      }
    }
  }
  if state.current_stmts.len() > 1 {
    return Result::Err(ParseError::UnclosedLeftBracket);
  }
  Result::Ok(Program {
    body: state.current_stmts.pop().unwrap(),
  })
}

#[test]
fn test_bracket() {
  matches!(parse("[[]"), Result::Err(ParseError::UnclosedLeftBracket));
  matches!(
    parse("[[]]]"),
    Result::Err(ParseError::UnexceptedTokenRightBracket)
  );
}
