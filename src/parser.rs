use crate::Error;
use crate::token::Token;
use crate::Pattern;
pub struct Parser {
    text: Option<String>,
    result: Vec<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            text: None,
            result: Vec::new(),
        }
    }

    pub fn parse(mut self, input: &str) -> Result<Pattern, Error> {
        if input.is_empty() {
            return Err(Error::new("Input pattern is empty"));
        }
        for c in input.chars() {
            match c {
                '*' => {
                    if let Some(text) = self.text {
                        self.result.push(Token::Text(text));
                    }
                    self.text = None;
                    self.result.push(Token::Any);
                },
                _ => {
                    let mut is_text= false;
                    if let Some(last_token) = self.result.last() {
                        is_text = last_token.is_text();
                    }
                    if self.result.is_empty() || !is_text {
                        self.result.push(Token::Text(String::new()));
                    }
                    if let Some(ref mut last_token) = self.result.last_mut() {
                        last_token.append_char(c)?;
                    } else {
                        return Err(Error::new("Invalid simple parser state. Unable to find last token."));
                    }
                },
            }
        }
        self.result.dedup_by(|a, b| {
            if a.is_any() && b.is_any() {
                return true;
            }
            return false;
        });
        return Ok(Pattern {tokens: self.result,});
    }
}