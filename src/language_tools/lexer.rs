use crate::language_tools::tokens::{Position, Token, TokenWithPosition};
use std::str::Chars;
use std::iter::Peekable;


pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
            position: Position { line: 1, column: 1 },
        }
    }

    pub fn next_token(&mut self) -> Option<Result<TokenWithPosition, String>> {
        self.skip_whitespace();

        let start_position = self.position.clone();

        match self.input.next() {
            Some(c) => {
                let token = match c {
                    '{' => Ok(Token::LeftBrace),
                    '}' => Ok(Token::RightBrace),
                    '[' => Ok(Token::LeftBracket),
                    ']' => Ok(Token::RightBracket),
                    ':' => Ok(Token::Colon),
                    ',' => Ok(Token::Comma),
                    '"' => self.tokenize_string(),
                    '-' | '0'..='9' => self.tokenize_number(c),
                    't' | 'f' => self.tokenize_boolean(c),
                    'n' => self.tokenize_null(),
                    _ => Err(format!("Unexpected character: {} at {:?}", c, start_position)),
                };

                self.position.column += 1;
                Some(token.map(|t| TokenWithPosition { token: t, position: start_position }))
            }
            None => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.input.peek() {
            if c.is_whitespace() {
                if *c == '\n' {
                    self.position.line += 1;
                    self.position.column = 1;
                } else {
                    self.position.column += 1;
                }
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn tokenize_string(&mut self) -> Result<Token, String> {
        let mut string = String::new();
        while let Some(c) = self.input.next() {
            match c {
                '"' => return Ok(Token::String(string)),
                '\\' => {
                    if let Some(next) = self.input.next() {
                        string.push(match next {
                            '"' | '\\' | '/' => next,
                            'b' => '\x08',
                            'f' => '\x0c',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'u' => self.parse_unicode_escape()?,
                            _ => return Err(format!("Invalid escape character: {} at {:?}", next, self.position)),
                        });
                    }
                }
                '\x00'..='\x1F' => return Err(format!("Invalid control character in string at {:?}", self.position)),
                _ => string.push(c),
            }
            self.position.column += 1;
        }
        Err("Unterminated string".to_string())
    }

    fn parse_unicode_escape(&mut self) -> Result<char, String> {
        let mut code_point = 0;
        for _ in 0..4 {
            if let Some(c) = self.input.next() {
                self.position.column += 1;
                code_point = (code_point << 4) | c.to_digit(16).ok_or_else(|| format!("Invalid Unicode escape at {:?}", self.position))? as u32;
            } else {
                return Err("Unexpected end of input in Unicode escape".to_string());
            }
        }
        char::from_u32(code_point).ok_or_else(|| "Invalid Unicode code point".to_string())
    }

    fn tokenize_number(&mut self, first_char: char) -> Result<Token, String> {
        let mut number = String::new();
        number.push(first_char);

        let mut has_decimal = false;
        let mut has_exponent = false;

        while let Some(&c) = self.input.peek() {
            match c {
                '0'..='9' => {
                    number.push(c);
                    self.input.next();
                    self.position.column += 1;
                }
                '.' if !has_decimal && !has_exponent => {
                    has_decimal = true;
                    number.push(c);
                    self.input.next();
                    self.position.column += 1;
                }
                'e' | 'E' if !has_exponent => {
                    has_exponent = true;
                    number.push(c);
                    self.input.next();
                    self.position.column += 1;
                    if let Some(&next) = self.input.peek() {
                        if next == '+' || next == '-' {
                            number.push(next);
                            self.input.next();
                            self.position.column += 1;
                        }
                    }
                }
                _ => break,
            }
        }

        if number.starts_with("0") && number.len() > 1 && number.chars().nth(1).unwrap().is_digit(10) {
            return Err("Numbers cannot have leading zeros".to_string());
        }

        number.parse::<f64>().map(Token::Number).map_err(|e| e.to_string())
    }

    fn tokenize_boolean(&mut self, first_char: char) -> Result<Token, String> {
        let expected = if first_char == 't' { "rue" } else { "alse" };
        for expected_char in expected.chars() {
            match self.input.next() {
                Some(c) if c == expected_char => self.position.column += 1,
                _ => return Err(format!("Invalid boolean at {:?}", self.position)),
            }
        }
        Ok(Token::Boolean(first_char == 't'))
    }

    fn tokenize_null(&mut self) -> Result<Token, String> {
        for expected_char in "ull".chars() {
            match self.input.next() {
                Some(c) if c == expected_char => self.position.column += 1,
                _ => return Err(format!("Invalid null at {:?}", self.position)),
            }
        }
        Ok(Token::Null)
    }
}