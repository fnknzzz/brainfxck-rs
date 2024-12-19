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

#[derive(Debug)]
#[allow(dead_code)]
pub struct LexerError {
  pub message: String,
  pub position: usize,
}

#[allow(dead_code)]
pub struct Lexer<'a> {
  pub source: &'a str,
  pub tokens: Vec<Token>,
}

fn tokenize<'a>(source: &'a str) -> Result<Vec<Token>, LexerError> {
  source
    .chars()
    .enumerate()
    .filter_map(|(pos, c)| match c {
      '<' => Some(Ok(Token::LessThan)),
      '>' => Some(Ok(Token::GreaterThan)),
      '+' => Some(Ok(Token::Plus)),
      '-' => Some(Ok(Token::Minus)),
      '.' => Some(Ok(Token::Dot)),
      ',' => Some(Ok(Token::Comma)),
      '[' => Some(Ok(Token::LeftBrackets)),
      ']' => Some(Ok(Token::RightBrackes)),
      // 忽略空白字符
      ' ' | '\t' | '\n' | '\r' => None,
      // 其他字符返回错误
      c => Some(Err(LexerError {
        message: format!("unexpected token '{}'", c),
        position: pos,
      })),
    })
    .collect() // collect 会自动处理 Result，如果有任何 Err 就返回第一个错误
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Result<Lexer<'a>, LexerError> {
    Ok(Lexer {
      source,
      tokens: tokenize(source)?,
    })
  }
}
