use crate::keywords::Keyword;
use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Identifier(&'a str),
    QuotedIdentifier(&'a str),
    StringLiteral(&'a str),
    NumberLiteral(&'a str),
    LocalVariable(&'a str),
    Comment(&'a str),
    Keyword(Keyword),
    Comma,
    LeftParen,
    RightParen,
    Equal,
    BangEqual,
    LessThanGreaterThan,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Plus,
    Minus,
    ForwardSlash,
    Asterisk,
    Percent,
    Period,
    SemiColon,
    Eof,
    // PlusEqual,
    // MinusEqual,
    // DivideEqual,
    // MultiplyEqual,
    // PercentEqual,
    // AndEqual,
    // OrEqual,
    // CaretEqual,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(i) => write!(f, "{}", i),
            Token::QuotedIdentifier(i) => write!(f, "{}", i),
            Token::StringLiteral(s) => write!(f, "{}", s),
            Token::NumberLiteral(n) => write!(f, "{}", n),
            Token::LocalVariable(v) => write!(f, "{}", v),
            Token::Comment(c) => write!(f, "-- {}", c),
            Token::Keyword(k) => write!(f, "{}", k),
            Token::Comma => f.write_str(","),
            Token::LeftParen => f.write_str("("),
            Token::RightParen => f.write_str(")"),
            Token::Equal => f.write_str("="),
            Token::BangEqual => f.write_str("!="),
            Token::LessThanGreaterThan => f.write_str("<>"),
            Token::LessThan => f.write_str("<"),
            Token::LessThanEqual => f.write_str("<="),
            Token::GreaterThan => f.write_str(">"),
            Token::GreaterThanEqual => f.write_str(">="),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::ForwardSlash => f.write_str("/"),
            Token::Asterisk => f.write_str("*"),
            Token::Percent => f.write_str("%"),
            Token::Period => f.write_str("."),
            Token::SemiColon => f.write_str(";"),
            // Token::LeftBracket => f.write_str("["),
            // Token::RightBracket => f.write_str("]"),
            // Token::LeftBrace => f.write_str("{"),
            // Token::RightBrace => f.write_str("}"),
            Token::Eof => f.write_str("EOF"),
            // Token::PlusEqual => f.write_str("+="),
            // Token::MinusEqual => f.write_str("-="),
            // Token::DivideEqual => f.write_str("/="),
            // Token::MultiplyEqual => f.write_str("*="),
            // Token::PercentEqual => f.write_str("%="),
            // Token::AndEqual => f.write_str("&="),
            // Token::OrEqual => f.write_str("|="),
            // Token::CaretEqual => f.write_str("^="),
        }
    }
}
