use std::collections::HashMap;
use crate::language_tools::lexer::Lexer;
use crate::language_tools::tokens::{Token, TokenWithPosition};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Result<TokenWithPosition, String>>,
    depth: usize,
}

const MAX_DEPTH: usize = 18;

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser { lexer, current_token, depth: 0 }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        match &self.current_token {
            Some(Ok(TokenWithPosition { token: Token::LeftBrace, .. })) => {
                let value = self.parse_object()?;
                if self.current_token.is_some() {
                    Err("Unexpected tokens after JSON value".to_string())
                } else {
                    Ok(value)
                }
            },
            Some(Ok(TokenWithPosition { token: Token::LeftBracket, .. })) => {
                let value = self.parse_array()?;
                if self.current_token.is_some() {
                    Err("Unexpected tokens after JSON value".to_string())
                } else {
                    Ok(value)
                }
            },
            Some(Ok(TokenWithPosition { token, position })) => {
                Err(format!("JSON payload should be an object or array, not {:?} at {:?}", token, position))
            },
            Some(Err(e)) => Err(e.clone()),
            None => Err("Empty input".to_string()),
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        if self.depth > MAX_DEPTH {
            return Err("Maximum nesting depth exceeded".to_string());
        }
        self.depth += 1;
        let result = match &self.current_token {
            Some(Ok(TokenWithPosition { token: Token::LeftBrace, .. })) => self.parse_object(),
            Some(Ok(TokenWithPosition { token: Token::LeftBracket, .. })) => self.parse_array(),
            Some(Ok(TokenWithPosition { token: Token::String(s), .. })) => {
                let value = JsonValue::String(s.clone());
                self.advance();
                Ok(value)
            }
            Some(Ok(TokenWithPosition { token: Token::Number(n), .. })) => {
                let value = JsonValue::Number(*n);
                self.advance();
                Ok(value)
            }
            Some(Ok(TokenWithPosition { token: Token::Boolean(b), .. })) => {
                let value = JsonValue::Boolean(*b);
                self.advance();
                Ok(value)
            }
            Some(Ok(TokenWithPosition { token: Token::Null, .. })) => {
                self.advance();
                Ok(JsonValue::Null)
            }
            Some(Ok(TokenWithPosition { token, position })) => {
                Err(format!("Unexpected token {:?} at {:?}", token, position))
            }
            Some(Err(e)) => Err(e.clone()),
            None => Err("Unexpected end of input".to_string()),
        };
        self.depth -= 1;
        result
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.advance(); // consume '{'
        let mut object = HashMap::new();

        if let Some(Ok(TokenWithPosition { token: Token::RightBrace, .. })) = &self.current_token {
            self.advance();
            return Ok(JsonValue::Object(object));
        }

        loop {
            let key = match &self.current_token {
                Some(Ok(TokenWithPosition { token: Token::String(s), .. })) => {
                    let key = s.clone();
                    self.advance();
                    key
                }
                Some(Ok(TokenWithPosition { token, position })) => {
                    return Err(format!("Expected string key, found {:?} at {:?}", token, position));
                }
                Some(Err(e)) => return Err(e.clone()),
                None => return Err("Unexpected end of input while parsing object key".to_string()),
            };

            match &self.current_token {
                Some(Ok(TokenWithPosition { token: Token::Colon, .. })) => self.advance(),
                Some(Ok(TokenWithPosition { token, position })) => {
                    return Err(format!("Expected ':', found {:?} at {:?}", token, position));
                }
                Some(Err(e)) => return Err(e.clone()),
                None => return Err("Unexpected end of input while parsing object".to_string()),
            }

            let value = self.parse_value()?;
            object.insert(key, value);

            match &self.current_token {
                Some(Ok(TokenWithPosition { token: Token::Comma, .. })) => {
                    self.advance();
                }
                Some(Ok(TokenWithPosition { token: Token::RightBrace, .. })) => {
                    self.advance();
                    break;
                }
                Some(Ok(TokenWithPosition { token, position })) => {
                    return Err(format!("Expected ',' or '}}', found {:?} at {:?}", token, position));
                }
                Some(Err(e)) => return Err(e.clone()),
                None => return Err("Unexpected end of input while parsing object".to_string()),
            }
        }

        Ok(JsonValue::Object(object))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.advance(); // consume '['
        let mut array = Vec::new();

        if let Some(Ok(TokenWithPosition { token: Token::RightBracket, .. })) = &self.current_token {
            self.advance();
            return Ok(JsonValue::Array(array));
        }

        loop {
            let value = self.parse_value()?;
            array.push(value);

            match &self.current_token {
                Some(Ok(TokenWithPosition { token: Token::Comma, .. })) => {
                    self.advance();
                }
                Some(Ok(TokenWithPosition { token: Token::RightBracket, .. })) => {
                    self.advance();
                    break;
                }
                Some(Ok(TokenWithPosition { token, position })) => {
                    return Err(format!("Expected ',' or ']', found {:?} at {:?}", token, position));
                }
                Some(Err(e)) => return Err(e.clone()),
                None => return Err("Unexpected end of input while parsing array".to_string()),
            }
        }

        Ok(JsonValue::Array(array))
    }
}