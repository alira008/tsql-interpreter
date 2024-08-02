use crate::ast::Span;
use crate::error::LexicalError;
use crate::error::LexicalErrorType;
use crate::keywords;
use crate::token_new::Token;
use crate::token_new::TokenKind;

pub type LexerResult<'a> = Result<Token<'a>, LexicalError>;
pub type SpannedKeyword = (Span, keywords::Keyword);

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    current_position: usize, // current position in input (points to current char)
    read_position: usize,    // current reading position in input (after current char)
    ch: Option<char>,        // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input,
            chars: input.chars().peekable(),
            current_position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn has_more_tokens(&self) -> bool {
        self.read_position < self.input.len()
    }

    fn read_char(&mut self) {
        if self.has_more_tokens() {
            self.ch = self.chars.next();
        } else {
            self.ch = None;
        }

        self.current_position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_some_and(|ch| ch.is_whitespace()) {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> &'a str {
        let start = self.current_position;
        while self
            .chars
            .peek()
            .is_some_and(|ch| ch.is_alphanumeric() || ch == &'_')
        {
            self.read_char();
        }
        &self.input[start..self.current_position + 1]
    }

    fn read_quoted_identifier(&mut self) -> Result<&'a str, LexicalError> {
        // skip the quote character
        self.read_char();
        // Read the string until the next single quote
        // current position is at the quote character
        let start = self.current_position;
        while self.chars.peek().is_some_and(|ch| ch != &']') {
            self.read_char();
        }

        // check if we ended on closing bracket
        if self.chars.peek().is_some_and(|ch| ch == &']') {
            // go to the closing bracket
            self.read_char();
            return Ok(&self.input[start..self.current_position]);
        }

        Err(LexicalError {
            error: LexicalErrorType::UnexpectedQuotedIdentifierEnd,
        })
    }

    fn read_string_literal(&mut self) -> Result<&'a str, LexicalError> {
        // skip the ' character
        self.read_char();
        // Read the string until the next single quote
        // current position is at the quote character
        let start = self.current_position;
        while self.chars.peek().is_some_and(|ch| ch != &'\'') {
            self.read_char();
        }

        // check if we ended on closing quote
        if self.chars.peek().is_some_and(|ch| ch == &'\'') {
            // read the closing quote
            self.read_char();
            return Ok(&self.input[start..self.current_position]);
        }

        Err(LexicalError {
            error: LexicalErrorType::UnexpectedStringEnd,
        })
    }

    fn read_comment(&mut self) -> &'a str {
        // skip the - character
        self.read_char();
        // Read the comment until the next new line
        let mut start = self.current_position;
        let mut end = self.current_position;
        let mut start_found = false;
        while self.chars.peek().is_some_and(|ch| ch != &'\n') {
            self.read_char();
            if self.ch.is_some_and(|ch| !ch.is_whitespace()) {
                if !start_found {
                    start = self.current_position;
                    start_found = true
                }
                end = self.current_position;
            }
        }
        // read the closing quote
        self.read_char();
        &self.input[start..end + 1]
    }

    fn read_number_literal(&mut self) -> &'a str {
        let start = self.current_position;
        // read all the digits
        while self.chars.peek().is_some_and(|ch| ch.is_numeric()) {
            self.read_char();
        }

        // check if the number has a period
        // aka number is a float
        if self.chars.peek() == Some(&'.') {
            // go to the period
            self.read_char();

            while self.chars.peek().is_some_and(|ch| ch.is_numeric()) {
                self.read_char();
            }
        }

        &self.input[start..self.current_position + 1]
    }

    fn next_lex(&mut self) -> LexerResult<'a> {
        self.skip_whitespace();

        let start = self.current_position as u32;
        let kind: TokenKind<'_> = match self.ch {
            Some(ch) => match ch {
                ',' => TokenKind::Comma,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '=' => TokenKind::Equal,
                '!' if self.chars.peek().is_some_and(|c| c == &'=') => TokenKind::BangEqual,
                '<' if self.chars.peek().is_some_and(|c| c == &'=') => TokenKind::LessThanEqual,
                '>' if self.chars.peek().is_some_and(|c| c == &'=') => TokenKind::GreaterThanEqual,
                '<' if self.chars.peek().is_some_and(|c| c == &'>') => TokenKind::LessThanGreaterThan,
                '<' => TokenKind::LessThan,
                '>' => TokenKind::GreaterThan,
                '+' => TokenKind::Plus,
                '-' if self.chars.peek().is_some_and(|c| c == &'-') => {
                    self.read_char();
                    let comment = self.read_comment();
                    TokenKind::Comment(comment)
                }
                '-' => TokenKind::Minus,
                '/' => TokenKind::ForwardSlash,
                '*' => TokenKind::Asterisk,
                '%' => TokenKind::Percent,
                '.' => TokenKind::Period,
                ';' => TokenKind::SemiColon,
                '[' if self.chars.peek().is_some_and(|c| c.is_alphabetic()) => {
                    match self.read_quoted_identifier() {
                        Ok(ident) => TokenKind::QuotedIdentifier(ident),
                        Err(error) => {
                            self.read_char();
                            return Err(error);
                        }
                    }
                }
                '\'' => match self.read_string_literal() {
                    Ok(string_literal) => TokenKind::StringLiteral(string_literal),
                    Err(error) => {
                        self.read_char();
                        return Err(error);
                    }
                },
                '@' if self.chars.peek().is_some_and(|c| c.is_alphabetic()) => {
                    self.read_char();

                    let local_variable = self.read_identifier();
                    TokenKind::LocalVariable(local_variable)
                }
                c if c.is_alphabetic() => {
                    let identifier = self.read_identifier();
                    if let Some(keyword) = keywords::lookup_keyword(identifier) {
                        TokenKind::Keyword(keyword)
                    } else {
                        TokenKind::Identifier(identifier)
                    }
                }
                '_' if self.chars.peek().is_some_and(|c| c.is_alphabetic()) => {
                    let identifier = self.read_identifier();
                    TokenKind::Identifier(identifier)
                }
                c if c.is_numeric() => {
                    let number_literal = self.read_number_literal();
                    TokenKind::NumberLiteral(number_literal)
                }
                _ => {
                    self.read_char();
                    return Err(LexicalError {
                        error: LexicalErrorType::UnrecognizedToken,
                    });
                }
            },
            None => TokenKind::Eof,
        };

        let location = Span::new(start, self.current_position as u32);
        self.read_char();
        Ok(Token::new(kind, location))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_token = self.next_lex();

        match next_token {
            Ok(token) => {
                if matches!(token.kind_as_ref(), TokenKind::Eof) {
                    None
                } else {
                    Some(next_token)
                }
            }
            Err(_) => Some(next_token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::keywords::Keyword;
    use super::*;

    #[test]
    fn test_random_tokens() {
        let input = "DesC current , preceding ;    .";
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        for result in lexer {
            let token = result.unwrap();
            tokens.push(token.kind());
        }
        let expected_tokens = vec![
            TokenKind::Keyword(Keyword::DESC),
            TokenKind::Keyword(Keyword::CURRENT),
            TokenKind::Comma,
            TokenKind::Keyword(Keyword::PRECEDING),
            TokenKind::SemiColon,
            TokenKind::Period,
        ];

        assert_eq!(expected_tokens, tokens);
    }

    #[test]
    fn test_identifiers_unquoted() {
        let input = "select name, id from users";
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        for result in lexer {
            let token = result.unwrap();
            tokens.push(token.kind());
        }

        let expected_tokens = vec![
            TokenKind::Keyword(Keyword::SELECT),
            TokenKind::Identifier("name"),
            TokenKind::Comma,
            TokenKind::Identifier("id"),
            TokenKind::Keyword(Keyword::FROM),
            TokenKind::Identifier("users"),
        ];

        assert_eq!(expected_tokens, tokens);
    }

    #[test]
    fn test_identifiers_quoted() {
        let input = "select [name], @hello id from users";
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        for result in lexer {
            let token = result.unwrap();
            tokens.push(token.kind());
        }

        let expected_tokens = vec![
            TokenKind::Keyword(Keyword::SELECT),
            TokenKind::QuotedIdentifier("name"),
            TokenKind::Comma,
            TokenKind::LocalVariable("hello"),
            TokenKind::Identifier("id"),
            TokenKind::Keyword(Keyword::FROM),
            TokenKind::Identifier("users"),
        ];

        assert_eq!(expected_tokens, tokens);
    }

    #[test]
    fn test_string() {
        let input = "select name as 'SuperName', id from users";
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        for result in lexer {
            let token = result.unwrap();
            tokens.push(token.kind());
        }

        let expected_tokens = vec![
            TokenKind::Keyword(Keyword::SELECT),
            TokenKind::Identifier("name"),
            TokenKind::Keyword(Keyword::AS),
            TokenKind::StringLiteral("SuperName"),
            TokenKind::Comma,
            TokenKind::Identifier("id"),
            TokenKind::Keyword(Keyword::FROM),
            TokenKind::Identifier("users"),
        ];

        assert_eq!(expected_tokens, tokens);
    }

    #[test]
    fn test_comment() {
        let input = "select name as 'SuperName',-- yes id \nfrom users";
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        for result in lexer {
            let token = result.unwrap();
            tokens.push(token.kind());
        }

        let expected_tokens = vec![
            TokenKind::Keyword(Keyword::SELECT),
            TokenKind::Identifier("name"),
            TokenKind::Keyword(Keyword::AS),
            TokenKind::StringLiteral("SuperName"),
            TokenKind::Comma,
            TokenKind::Comment("yes id"),
            TokenKind::Keyword(Keyword::FROM),
            TokenKind::Identifier("users"),
        ];

        assert_eq!(expected_tokens, tokens);
    }

    #[test]
    fn test_illegal_string_literal() {
        let input = "select name as 'SuperName, yess id from users";
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        for result in lexer {
            tokens.push(result.map(|t| t.kind()));
        }

        let expected_tokens = vec![
            Ok(TokenKind::Keyword(Keyword::SELECT)),
            Ok(TokenKind::Identifier("name")),
            Ok(TokenKind::Keyword(Keyword::AS)),
            Err(LexicalError {
                error: LexicalErrorType::UnexpectedStringEnd,
            }),
        ];

        assert_eq!(expected_tokens, tokens);
    }

    #[test]
    fn test_illegal_quoted_identifier() {
        let input = "select name as [SuperName, yess id from users";
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        for result in lexer {
            tokens.push(result.map(|t| t.kind()));
        }

        let expected_tokens = vec![
            Ok(TokenKind::Keyword(Keyword::SELECT)),
            Ok(TokenKind::Identifier("name")),
            Ok(TokenKind::Keyword(Keyword::AS)),
            Err(LexicalError {
                error: LexicalErrorType::UnexpectedQuotedIdentifierEnd,
            }),
        ];

        assert_eq!(expected_tokens, tokens);
    }
}
