use lexer::{LexicalError, Span, TokenKind};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError<'a> {
    pub error: ParseErrorType<'a>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseErrorType<'a> {
    UnexpectedToken {
        token: TokenKind<'a>,
        expected: Vec<String>,
    },
    UnrecognizedEof,
    ExpectedKeyword,
    ExpectedFunctionName,
    EmptySelectColumns,
    EmptyGroupByClause,
    EmptyPartitionByClause,
    EmptyOrderByArgs,
    ExpectedDataType,
    ExpectedDataTypeSize,
    ExpectedComparisonOperator,
    ExpectedArithmeticOperator,
    ExpectedUnaryOperator,
    ExpectedSubqueryOrExpressionList,
    MissingRowsOrRangeInWindowFrameClause,
    MissingAliasAfterAsKeyword,
    ExpectedUnboundedPrecedingCurrentRowOrNumberPreceding,
    ExpectedUnboundedFollowingCurrentRowOrNumberFollowing,
    ExpectedLocalVariable,
    ExpectedObjectToInsertTo,
    InvalidOrUnimplementedStatement,
    LexerError {
        error: LexicalError,
    },
}

pub fn parse_error<T>(error: ParseErrorType, span: Span) -> Result<T, ParseError> {
    Err(ParseError { error, span })
}

pub fn parse_lexical_error<'a>(error: LexicalError) -> ParseError<'a> {
    ParseError {
        error: ParseErrorType::LexerError { error },
        span: error.span,
    }
}

impl<'a> ParseError<'a> {
    pub fn location(&self, input: &'a str) -> String {
        let mut line_number: u32 = 1;
        let mut column_number: u32 = 1;

        for (i, c) in input.char_indices() {
            if i == self.span.start as usize {
                break;
            }

            if c == '\n' {
                line_number += 1;
                column_number = 1;
            } else {
                column_number += 1;
            }
        }

        format!("line: {} col: {}", line_number, column_number)
    }

    pub fn details(&self) -> String {
        match &self.error {
            ParseErrorType::UnexpectedToken { token, expected } => {
                let found = match token {
                    TokenKind::Identifier(_) => "an identifier".into(),
                    TokenKind::QuotedIdentifier(_) => "a quoted identifier".into(),
                    TokenKind::StringLiteral(_) => "a string".into(),
                    TokenKind::NumberLiteral(_) => "a number".into(),
                    TokenKind::LocalVariable(_) => "a local variable".into(),
                    TokenKind::Comment(_) => "a comment".into(),
                    TokenKind::Eof => "end of file".into(),
                    TokenKind::Comma
                    | TokenKind::LeftParen
                    | TokenKind::RightParen
                    | TokenKind::Equal
                    | TokenKind::BangEqual
                    | TokenKind::LessThanGreaterThan
                    | TokenKind::LessThan
                    | TokenKind::LessThanEqual
                    | TokenKind::GreaterThan
                    | TokenKind::GreaterThanEqual
                    | TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::ForwardSlash
                    | TokenKind::Asterisk
                    | TokenKind::PercentSign
                    | TokenKind::Period
                    | TokenKind::SemiColon => token.to_string(),
                    _ => format!("the keyword {}", token),
                };
                let message = std::iter::once(format!(
                    "I was not expecting this. Found {found}, expected one of: "
                ))
                .chain(expected.iter().map(|s| format!("- {} ", s)))
                .collect();

                message
            }
            ParseErrorType::UnrecognizedEof => "I was not expecting an end of file".into(),
            ParseErrorType::ExpectedKeyword => "I was exepecting a keyword".into(),
            ParseErrorType::ExpectedFunctionName => "I expected a function name".into(),
            ParseErrorType::EmptySelectColumns => "I expected columns to select from table".into(),
            ParseErrorType::EmptyGroupByClause => "I expected a group by clause".into(),
            ParseErrorType::EmptyPartitionByClause => "I expected a partition by clause".into(),
            ParseErrorType::ExpectedComparisonOperator => "I expected a comparison operator".into(),
            ParseErrorType::ExpectedArithmeticOperator => "I expected an arithmetic operator".into(),
            ParseErrorType::ExpectedUnaryOperator => "I expected a unary operator".into(),
            ParseErrorType::EmptyOrderByArgs => "I expected columns to order by".into(),
            ParseErrorType::ExpectedDataType => "I expected a data type".into(),
            ParseErrorType::ExpectedDataTypeSize => "I expected a float precision".into(),
            ParseErrorType::ExpectedSubqueryOrExpressionList => {
                "I expected subquery or expression list".into()
            }
            ParseErrorType::MissingRowsOrRangeInWindowFrameClause => {
                "I expected rows or range in window frame clause".into()
            }
            ParseErrorType::MissingAliasAfterAsKeyword => {
                "I expected an alias after as keyword".into()
            }
            ParseErrorType::ExpectedUnboundedPrecedingCurrentRowOrNumberPreceding => {
                "I expected unbounded preceding current row or number preceding".into()
            }
            ParseErrorType::ExpectedUnboundedFollowingCurrentRowOrNumberFollowing => {
                "I expected unbounded following current row or number following".into()
            }
            ParseErrorType::ExpectedLocalVariable => "I expected a local variable".into(),
            ParseErrorType::ExpectedObjectToInsertTo => {
                "I expected an object to insert into".into()
            }
            ParseErrorType::InvalidOrUnimplementedStatement => {
                "I was not expecting an invalid or a statement that is not implemented yet".into()
            }
            ParseErrorType::LexerError { error } => error.details(),
        }
    }
}
