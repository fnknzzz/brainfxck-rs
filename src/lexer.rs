pub enum Token {
  GreaterThan,
  LessThan,
  Plus,
  Minus,
  Dot,
  Comma,
  LeftBrackets,
  RightBrackes,
}

pub struct Lexer<'a> {
  pub source: &'a str,
  pub tokens: Vec<Token>,
}

fn tokenize<'a>(source: &'a str) -> Vec<Token> {
  source
    .chars()
    .map(|i| match i {
      '<' => Token::LessThan,
      '>' => Token::GreaterThan,
      '+' => Token::Plus,
      '-' => Token::Minus,
      '.' => Token::Dot,
      ',' => Token::Comma,
      '[' => Token::LeftBrackets,
      ']' => Token::RightBrackes,
      _ => panic!("unexcepted token: {}", i),
    })
    .collect()
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Lexer {
    Lexer {
      source,
      tokens: tokenize(source),
    }
  }
}
