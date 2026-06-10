#[derive(Debug, PartialEq, Eq)]
enum Keyword {
    Get
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Keyword(Keyword),
    Word(String)
}

#[derive(Debug)]
enum LexerError {
    EmptyText,
    ExhaustedInput
}

pub struct Lexer {
    chars: Vec<char>,
    text_len: usize,
    offset: usize
}

impl Lexer {
    pub fn new(text: &'static str) -> Result<Self, LexerError> {
        if text.len() == 0 {
            return Err(LexerError::EmptyText);
        }

        let chars = text.chars().into_iter().collect();


        Ok(Self {
            chars,
            text_len: text.len(),
            offset: 0
        })
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::with_capacity(10);

        while self.offset < self.text_len {
            let token = match self.chars[self.offset] {
                _ => self.parse_word_or_keyword()?
            };

            tokens.push(token);
        }


        Ok(tokens)
    }

    fn parse_word_or_keyword(&mut self) -> Result<Token, LexerError> {
        let mut word_chs = Vec::with_capacity(20);

        while self.offset < self.text_len && self.chars[self.offset] != ' ' {
           word_chs.push(self.chars[self.offset]); 
           self.offset += 1;
        }

        if word_chs.len() == 0 {
            return Err(LexerError::ExhaustedInput); 
        }

        let word: String = word_chs.iter().collect();

        // Check if the word is a keyword or not
        let token = match word.to_lowercase().as_str() {
            "get" => Token::Keyword(Keyword::Get),
            _ => Token::Word(word)
        };

        Ok(token)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_parse_get_keyword_token() {
        let query = "get";
        let mut lexer = Lexer::new(query).unwrap();
        
        let result = lexer.tokenize().unwrap();

        assert_eq!(vec![Token::Keyword(Keyword::Get)], result);
    }

}
