/*!
This crate provides a library for compiling and matching simple text patterns.

# Example
Passed pattern `some*text` will be compiled into equivalent regexp `^some.*text$`.

# Syntax
* `*` - one or more any symbol.
* `any other text` - interpreted as simple text.

# Usage
```rust
use simple_text_pattern::Pattern;
let pattern = Pattern::new("some*text").expect("Unable to compile pattern");
assert_eq!(true, pattern.is_match("sometext"));
assert_eq!(true, pattern.is_match("some text"));
assert_eq!(false, pattern.is_match("not some text"));
```

*/
mod error;
mod parser;
mod token;
pub use self::error::Error;
use self::token::Token;
use self::parser::Parser;

#[derive(Debug, Clone)]
/// Structure represents pattern.
pub struct Pattern {
    tokens: Vec<Token>,
}

impl Pattern {
    /// Parses input pattern.
    pub fn new(input: &str) -> Result<Pattern, Error> {
        let parser = Parser::new();
        let result = parser.parse(input)?;
        return Ok(result);
    }

    /// Checks is input string matches current pattern.
    pub fn is_match(&self, input: &str) -> bool {
        let mut inputs = vec![input];
        let mut may_skip_chars = false;
        for token in self.tokens.iter() {
            match token {
                &Token::Text(ref text) => {
                    inputs = Self::handle_text(inputs, text, may_skip_chars);
                    may_skip_chars = false;
                },
                &Token::Any => {
                    may_skip_chars = true;
                },
            }
        }
        if may_skip_chars && !inputs.is_empty() {
            return true;
        }
        for input in inputs.iter() {
            if input.is_empty() {
                return true;
            }
        }
        return false;
    }

    fn handle_text<'a>(inputs: Vec<&'a str>, need: &str, may_skip_chars: bool) -> Vec<&'a str> {
        let mut result = Vec::with_capacity(inputs.len());
        for input in inputs {
            let end_index = input.len();
            for (found_index, _) in input.match_indices(need) {
                if !may_skip_chars && found_index != 0 {
                    continue;
                }
                let out_index = found_index + need.len();
                if out_index >= end_index {
                    result.push("");
                } else {
                    result.push(&input[out_index..end_index]);
                }
            }
        }
        result.sort();
        result.dedup_by(|a, b| {
            return a == b;
        });
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::Pattern;

    #[test]
    fn test_empty_pattern() {
        assert_eq!(true, Pattern::new("").is_err());
    }

    #[test]
    fn test_text_pattern() {
        let pattern = Pattern::new("abcdef").expect("Unable to build text pattern");
        assert_eq!(false, pattern.is_match(""));
        assert_eq!(true, pattern.is_match("abcdef"));
        assert_eq!(false, pattern.is_match("1abcdef"));
        assert_eq!(false, pattern.is_match("abcdef1"));
        assert_eq!(false, pattern.is_match("abc"));
    }

    #[test]
    fn test_any_pattern() {
        let pattern = Pattern::new("*").expect("Unable to build any pattern");
        assert_eq!(true, pattern.is_match(""));
        assert_eq!(true, pattern.is_match("abcdef"));
        assert_eq!(true, pattern.is_match("1abcdef"));
        assert_eq!(true, pattern.is_match("abcdef1"));
        assert_eq!(true, pattern.is_match("abc"));
    }

    #[test]
    fn test_text_n_any_pattern() {
        let pattern = Pattern::new("abc*").expect("Unable to build any pattern with text pattern");
        assert_eq!(false, pattern.is_match(""));
        assert_eq!(true, pattern.is_match("abcdef"));
        assert_eq!(false, pattern.is_match("1abcdef"));
        assert_eq!(true, pattern.is_match("abcdef1"));
        assert_eq!(true, pattern.is_match("abc"));
        assert_eq!(false, pattern.is_match("q"));
        assert_eq!(false, pattern.is_match("qwe"));

        let pattern = Pattern::new("*abc").expect("Unable to build any pattern with text pattern");
        assert_eq!(false, pattern.is_match(""));
        assert_eq!(false, pattern.is_match("abcdef"));
        assert_eq!(false, pattern.is_match("1abcdef"));
        assert_eq!(false, pattern.is_match("abcdef1"));
        assert_eq!(true, pattern.is_match("abc"));
        assert_eq!(true, pattern.is_match("svsdfvsdfabc"));

        let pattern = Pattern::new("abc*def").expect("Unable to build any pattern with text pattern");
        assert_eq!(false, pattern.is_match(""));
        assert_eq!(true, pattern.is_match("abcdef"));
        assert_eq!(false, pattern.is_match("1abcdef"));
        assert_eq!(false, pattern.is_match("abcdef1"));
        assert_eq!(false, pattern.is_match("abc"));
        assert_eq!(true, pattern.is_match("abcabcdefdef"));
        assert_eq!(true, pattern.is_match("abc1def"));
        assert_eq!(true, pattern.is_match("abc1sdfvsdvdef"));
    }
}