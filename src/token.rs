use crate::Error;

#[derive(Debug, Clone)]
pub enum Token {
    Any,
    Text(String),
}

impl Token {
    pub (crate) fn is_text(&self) -> bool {
        match self {
            &Token::Text(..) => true,
            _ => false,
        }
    }

    pub (crate) fn is_any(&self) -> bool {
        match self {
            &Token::Any => true,
            _ => false,
        }
    }

    pub (crate) fn append_char(&mut self, ch: char) -> Result<(), Error> {
        match self {
            &mut Token::Text(ref mut text) => {
                text.push(ch);
                return Ok(());
            },
            _ => {
                return Err(Error::new("Unable append char to non-text token"));
            },
        };
    }
}
