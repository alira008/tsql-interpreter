use super::{
    display_list_comma_separated, display_list_delimiter_separated, DataType, Keyword,
    SelectStatement,
};
use crate::error::{parse_error, ParseError, ParseErrorType};
use core::fmt;
use sql_lexer::{Span, Token, TokenKind};

#[derive(Debug, PartialEq, Clone)]
pub struct ComparisonOperator {
    pub location: Span,
    pub kind: ComparisonOperatorKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArithmeticOperator {
    pub location: Span,
    pub kind: ArithmeticOperatorKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperator {
    pub location: Span,
    pub kind: UnaryOperatorKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub location: Span,
    pub content: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OrderByArg {
    pub column: Expression,
    pub order_kw: Option<Keyword>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OverClause {
    pub over_kw: Keyword,
    pub partition_by_kws: Option<Vec<Keyword>>,
    pub partition_by: Vec<Expression>,
    pub order_by_kws: Option<Vec<Keyword>>,
    pub order_by: Vec<OrderByArg>,
    pub window_frame: Option<WindowFrame>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WindowFrame {
    pub rows_or_range: RowsOrRange,
    pub rows_or_range_kw: Keyword,
    pub start_bound_keywords: Vec<Keyword>,
    pub start: WindowFrameBound,
    pub between_kw: Option<Keyword>,
    pub and_kw: Option<Keyword>,
    pub end_bound_keywords: Option<Vec<Keyword>>,
    pub end: Option<WindowFrameBound>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Asterisk,
    Identifier(Literal),
    QuotedIdentifier(Literal),
    StringLiteral(Literal),
    NumberLiteral(Literal),
    LocalVariable(Literal),
    Keyword(Keyword),
    Compound(Vec<Expression>),
    Arithmetic {
        operator: ArithmeticOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    And {
        and_kw: Keyword,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Or {
        or_kw: Keyword,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Comparison {
        operator: ComparisonOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    Function {
        name: Box<FunctionName>,
        args: Option<Vec<Expression>>,
        over: Option<Box<OverClause>>,
    },
    Cast {
        cast_kw: Keyword,
        expression: Box<Expression>,
        as_kw: Keyword,
        data_type: DataType,
    },
    InExpressionList {
        test_expression: Box<Expression>,
        in_kw: Keyword,
        not_kw: Option<Keyword>,
        list: Vec<Expression>,
    },
    InSubquery {
        test_expression: Box<Expression>,
        in_kw: Keyword,
        not_kw: Option<Keyword>,
        subquery: Box<Expression>,
    },
    Subquery(Box<SelectStatement>),
    Between {
        test_expression: Box<Expression>,
        not_kw: Option<Keyword>,
        between_kw: Keyword,
        begin: Box<Expression>,
        and_kw: Keyword,
        end: Box<Expression>,
    },
    Not {
        not_kw: Keyword,
        expression: Box<Expression>,
    },
    Exists {
        exists_kw: Keyword,
        subquery: Box<Expression>,
    },
    All {
        all_kw: Keyword,
        scalar_expression: Box<Expression>,
        comparison_op: ComparisonOperator,
        subquery: Box<Expression>,
    },
    Some {
        some_kw: Keyword,
        scalar_expression: Box<Expression>,
        comparison_op: ComparisonOperator,
        subquery: Box<Expression>,
    },
    Any {
        any_kw: Keyword,
        scalar_expression: Box<Expression>,
        comparison_op: ComparisonOperator,
        subquery: Box<Expression>,
    },
    Like {
        match_expression: Box<Expression>,
        not_kw: Option<Keyword>,
        like_kw: Keyword,
        pattern: Box<Expression>,
    },
    SimpleCase {
        case_kw: Keyword,
        input_expression: Box<Expression>,
        conditions: Vec<CaseCondition>,
        end_kw: Keyword,
    },
    SearchedCase {
        case_kw: Keyword,
        conditions: Vec<CaseCondition>,
        end_kw: Keyword,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComparisonOperatorKind {
    Equal,
    NotEqualBang,
    NotEqualArrow,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ArithmeticOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperatorKind {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionName {
    Builtin(Keyword),
    User(Expression),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RowsOrRange {
    Rows,
    Range,
}

#[derive(Debug, PartialEq, Clone)]
pub enum WindowFrameBound {
    CurrentRow,
    Preceding(Expression),
    Following(Expression),
    UnboundedPreceding,
    UnboundedFollowing,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CaseCondition {
    WhenCondition {
        when_kw: Keyword,
        when_expression: Expression,
        then_kw: Keyword,
        result_expression: Expression,
    },
    ElseCondition {
        else_kw: Keyword,
        result_expression: Expression,
    },
}

impl ComparisonOperator {
    pub fn new(location: Span, kind: ComparisonOperatorKind) -> Self {
        Self { location, kind }
    }
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for ComparisonOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComparisonOperatorKind::Equal => f.write_str("="),
            ComparisonOperatorKind::NotEqualBang => f.write_str("!="),
            ComparisonOperatorKind::NotEqualArrow => f.write_str("<>"),
            ComparisonOperatorKind::GreaterThan => f.write_str(">"),
            ComparisonOperatorKind::GreaterThanEqual => f.write_str(">="),
            ComparisonOperatorKind::LessThan => f.write_str("<="),
            ComparisonOperatorKind::LessThanEqual => f.write_str("<="),
        }
    }
}

impl<'a> TryFrom<Token<'a>> for ComparisonOperator {
    type Error = ParseError<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        let kind = match value.kind() {
            TokenKind::Equal => ComparisonOperatorKind::Equal,
            TokenKind::BangEqual => ComparisonOperatorKind::NotEqualBang,
            TokenKind::LessThanGreaterThan => ComparisonOperatorKind::NotEqualArrow,
            TokenKind::GreaterThan => ComparisonOperatorKind::GreaterThan,
            TokenKind::GreaterThanEqual => ComparisonOperatorKind::GreaterThanEqual,
            TokenKind::LessThan => ComparisonOperatorKind::LessThan,
            TokenKind::LessThanEqual => ComparisonOperatorKind::LessThanEqual,
            _ => return parse_error(ParseErrorType::ExpectedKeyword),
        };
        Ok(Self::new(value.location(), kind))
    }
}

impl<'a> TryFrom<Option<Token<'a>>> for ComparisonOperator {
    type Error = ParseError<'a>;

    fn try_from(value: Option<Token<'a>>) -> Result<Self, Self::Error> {
        if let Some(token) = value {
            ComparisonOperator::try_from(token)
        } else {
            unreachable!()
        }
    }
}

impl ArithmeticOperator {
    pub fn new(location: Span, kind: ArithmeticOperatorKind) -> Self {
        Self { location, kind }
    }
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for ArithmeticOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticOperatorKind::Plus => f.write_str("+"),
            ArithmeticOperatorKind::Minus => f.write_str("-"),
            ArithmeticOperatorKind::Multiply => f.write_str("*"),
            ArithmeticOperatorKind::Divide => f.write_str("/"),
            ArithmeticOperatorKind::Modulus => f.write_str("%"),
        }
    }
}

impl<'a> TryFrom<Token<'a>> for ArithmeticOperator {
    type Error = ParseError<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        let kind = match value.kind() {
            TokenKind::Plus => ArithmeticOperatorKind::Plus,
            TokenKind::Minus => ArithmeticOperatorKind::Minus,
            TokenKind::Asterisk => ArithmeticOperatorKind::Multiply,
            TokenKind::ForwardSlash => ArithmeticOperatorKind::Divide,
            TokenKind::PercentSign => ArithmeticOperatorKind::Modulus,
            _ => return parse_error(ParseErrorType::ExpectedKeyword),
        };
        Ok(Self::new(value.location(), kind))
    }
}

impl<'a> TryFrom<Option<Token<'a>>> for ArithmeticOperator {
    type Error = ParseError<'a>;

    fn try_from(value: Option<Token<'a>>) -> Result<Self, Self::Error> {
        if let Some(token) = value {
            ArithmeticOperator::try_from(token)
        } else {
            unreachable!()
        }
    }
}

impl UnaryOperator {
    pub fn new(location: Span, kind: UnaryOperatorKind) -> Self {
        Self { location, kind }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for UnaryOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperatorKind::Plus => f.write_str("+"),
            UnaryOperatorKind::Minus => f.write_str("-"),
        }
    }
}

impl<'a> TryFrom<Token<'a>> for UnaryOperator {
    type Error = ParseError<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        let kind = match value.kind() {
            TokenKind::Plus => UnaryOperatorKind::Plus,
            TokenKind::Minus => UnaryOperatorKind::Minus,
            _ => return parse_error(ParseErrorType::ExpectedKeyword),
        };
        Ok(Self::new(value.location(), kind))
    }
}

impl<'a> TryFrom<Option<Token<'a>>> for UnaryOperator {
    type Error = ParseError<'a>;

    fn try_from(value: Option<Token<'a>>) -> Result<Self, Self::Error> {
        if let Some(token) = value {
            UnaryOperator::try_from(token)
        } else {
            unreachable!()
        }
    }
}

impl Literal {
    pub fn new(location: Span, content: String) -> Self {
        Self { location, content }
    }
}

impl<'a> TryFrom<Token<'a>> for Literal {
    type Error = ParseError<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        let content = match value.kind() {
            TokenKind::Identifier(str)
            | TokenKind::QuotedIdentifier(str)
            | TokenKind::NumberLiteral(str)
            | TokenKind::StringLiteral(str)
            | TokenKind::LocalVariable(str) => str.to_string(),
            _ => return parse_error(ParseErrorType::ExpectedKeyword),
        };
        Ok(Self::new(value.location(), content))
    }
}

impl<'a> TryFrom<Option<Token<'a>>> for Literal {
    type Error = ParseError<'a>;

    fn try_from(value: Option<Token<'a>>) -> Result<Self, Self::Error> {
        if let Some(token) = value {
            Literal::try_from(token)
        } else {
            unreachable!()
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<'a> TryFrom<Token<'a>> for Expression {
    type Error = ParseError<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        let expr = match value.kind() {
            TokenKind::Identifier(_) => Expression::Identifier(Literal::try_from(value)?),
            TokenKind::QuotedIdentifier(_) => {
                Expression::QuotedIdentifier(Literal::try_from(value)?)
            }
            TokenKind::NumberLiteral(_) => Expression::NumberLiteral(Literal::try_from(value)?),
            TokenKind::StringLiteral(_) => Expression::StringLiteral(Literal::try_from(value)?),
            TokenKind::LocalVariable(_) => Expression::LocalVariable(Literal::try_from(value)?),
            TokenKind::Asterisk => Expression::Asterisk,
            _ => return parse_error(ParseErrorType::ExpectedKeyword),
        };
        Ok(expr)
    }
}

impl<'a> TryFrom<Option<Token<'a>>> for Expression {
    type Error = ParseError<'a>;

    fn try_from(value: Option<Token<'a>>) -> Result<Self, Self::Error> {
        if let Some(token) = value {
            Expression::try_from(token)
        } else {
            unreachable!()
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Asterisk => write!(f, "*"),
            Expression::Identifier(v) => write!(f, "{}", v),
            Expression::QuotedIdentifier(v) => write!(f, "[{}]", v),
            Expression::StringLiteral(v) => write!(f, "'{}'", v),
            Expression::NumberLiteral(v) => write!(f, "{}", v),
            Expression::LocalVariable(v) => write!(f, "@{}", v),
            Expression::Keyword(v) => write!(f, "{}", v),
            Expression::Compound(v) => display_list_delimiter_separated(v, ".", f),
            Expression::Arithmetic {
                operator,
                left,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::Comparison {
                operator,
                left,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::Unary { operator, right } => write!(f, "{} {}", operator, right),
            Expression::And {
                and_kw,
                left,
                right,
            } => write!(f, "{} {} {}", left, and_kw, right),
            Expression::Or { or_kw, left, right } => write!(f, "{} {} {}", left, or_kw, right),
            Expression::Function { name, args, over } => {
                write!(f, "{}", name)?;
                f.write_str("(")?;
                if let Some(args_vec) = args {
                    display_list_comma_separated(args_vec, f)?;
                }
                f.write_str(")")?;
                if let Some(over_clause) = over {
                    write!(f, "{}", over_clause)?;
                }
                Ok(())
            }
            Expression::Cast {
                cast_kw,
                expression,
                as_kw,
                data_type,
            } => write!(f, "{}({} {} {})", cast_kw, expression, as_kw, data_type),
            Expression::InExpressionList {
                test_expression,
                in_kw,
                not_kw,
                list,
            } => {
                write!(f, "{}", test_expression)?;
                if let Some(kw) = not_kw {
                    write!(f, " {}", kw)?;
                }
                write!(f, " {}", in_kw)?;
                f.write_str(" (")?;
                display_list_comma_separated(list, f)?;
                f.write_str(")")?;

                Ok(())
            }
            Expression::Subquery(s) => {
                write!(f, "({})", s)
            }
            Expression::InSubquery {
                test_expression,
                in_kw,
                not_kw,
                subquery,
            } => {
                write!(f, "{}", test_expression)?;
                if let Some(kw) = not_kw {
                    write!(f, " {}", kw)?;
                }
                write!(f, " {} {}", in_kw, subquery)?;

                Ok(())
            }
            Expression::Between {
                test_expression,
                not_kw,
                between_kw,
                begin,
                and_kw,
                end,
            } => {
                write!(f, "{}", test_expression)?;
                if let Some(kw) = not_kw {
                    write!(f, " {}", kw)?;
                }
                write!(f, " {} {} {} {}", between_kw, begin, and_kw, end)?;

                Ok(())
            }
            Expression::Not { not_kw, expression } => {
                write!(f, "{} {}", not_kw, expression)?;

                Ok(())
            }
            Expression::Exists {
                exists_kw,
                subquery,
            } => {
                write!(f, "{} {}", exists_kw, subquery)?;

                Ok(())
            }
            Expression::All {
                all_kw,
                scalar_expression,
                comparison_op,
                subquery,
            } => write!(
                f,
                "{} {} {} {}",
                scalar_expression, comparison_op, all_kw, subquery
            ),
            Expression::Some {
                some_kw,
                scalar_expression,
                comparison_op,
                subquery,
            } => write!(
                f,
                "{} {} {} {}",
                scalar_expression, comparison_op, some_kw, subquery
            ),
            Expression::Any {
                any_kw,
                scalar_expression,
                comparison_op,
                subquery,
            } => write!(
                f,
                "{} {} {} {}",
                scalar_expression, comparison_op, any_kw, subquery
            ),
            Expression::Like {
                match_expression,
                not_kw,
                like_kw,
                pattern,
            } => {
                write!(f, "{}", match_expression)?;
                if let Some(not_kw) = not_kw {
                    write!(f, " {}", not_kw)?;
                }
                write!(f, " {} {}", like_kw, pattern)?;

                Ok(())
            }
            Expression::SimpleCase {
                case_kw,
                input_expression,
                conditions,
                end_kw,
            } => {
                write!(f, "{} {} ", case_kw, input_expression)?;
                display_list_delimiter_separated(conditions, " ", f)?;
                write!(f, " {}", end_kw)?;

                Ok(())
            }
            Expression::SearchedCase {
                case_kw,
                conditions,
                end_kw,
            } => {
                write!(f, "{} ", case_kw)?;
                display_list_delimiter_separated(conditions, " ", f)?;
                write!(f, " {}", end_kw)?;

                Ok(())
            }
        }
    }
}

impl fmt::Display for CaseCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            CaseCondition::WhenCondition {
                when_kw,
                when_expression,
                then_kw,
                result_expression,
            } => write!(
                f,
                "{} {} {} {}",
                when_kw, when_expression, then_kw, result_expression
            ),
            CaseCondition::ElseCondition {
                else_kw,
                result_expression,
            } => write!(f, "{} {}", else_kw, result_expression),
        }
    }
}

impl fmt::Display for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FunctionName::Builtin(e) => write!(f, "{}", e),
            FunctionName::User(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for WindowFrameBound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            WindowFrameBound::CurrentRow => write!(f, "CURRENT ROW"),
            WindowFrameBound::Preceding(expr) => write!(f, "{} PRECEDING", expr),
            WindowFrameBound::Following(expr) => write!(f, "{} FOLLOWING", expr),
            WindowFrameBound::UnboundedPreceding => write!(f, "UNBOUNDED PRECEDING"),
            WindowFrameBound::UnboundedFollowing => write!(f, "UNBOUNDED FOLLOWING"),
        }
    }
}

impl fmt::Display for RowsOrRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RowsOrRange::Rows => write!(f, "ROWS"),
            RowsOrRange::Range => write!(f, "RANGE"),
        }
    }
}

impl fmt::Display for OrderByArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.order_kw {
            Some(kw) => write!(f, "{} {}", self.column, kw),
            None => write!(f, "{}", self.column),
        }
    }
}

impl fmt::Display for WindowFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {}", self.rows_or_range_kw)?;
        if let Some(between_kw) = self.between_kw {
            write!(f, " {}", between_kw)?;
        }
        match &self.start {
            WindowFrameBound::Preceding(expr) | WindowFrameBound::Following(expr) => {
                write!(f, " {} ", expr)?;
                display_list_delimiter_separated(&self.start_bound_keywords, " ", f)?;
            }
            WindowFrameBound::CurrentRow
            | WindowFrameBound::UnboundedPreceding
            | WindowFrameBound::UnboundedFollowing => {
                f.write_str(" ")?;
                display_list_delimiter_separated(&self.start_bound_keywords, " ", f)?
            }
        }
        if let Some(and_kw) = self.and_kw {
            write!(f, " {}", and_kw)?;
        }

        if let (Some(end), Some(end_bound_keywords)) = (&self.end, &self.end_bound_keywords) {
            match end {
                WindowFrameBound::Preceding(expr) | WindowFrameBound::Following(expr) => {
                    write!(f, " {} ", expr)?;
                    display_list_delimiter_separated(&end_bound_keywords, " ", f)?;
                }
                WindowFrameBound::CurrentRow
                | WindowFrameBound::UnboundedPreceding
                | WindowFrameBound::UnboundedFollowing => {
                    f.write_str(" ")?;
                    display_list_delimiter_separated(&end_bound_keywords, " ", f)?
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for OverClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {}", self.over_kw)?;
        f.write_str("(")?;
        if let Some(partition_by_kws) = &self.partition_by_kws {
            display_list_delimiter_separated(&partition_by_kws, " ", f)?;
            f.write_str(" ")?;
        }
        if !self.partition_by.is_empty() {
            display_list_comma_separated(&self.partition_by, f)?;
        }

        if !self.partition_by.is_empty() && !self.order_by.is_empty() {
            f.write_str(" ")?;
        }

        if let Some(order_by_kws) = &self.order_by_kws {
            display_list_delimiter_separated(&order_by_kws, " ", f)?;
            f.write_str(" ")?;
        }
        if !self.order_by.is_empty() {
            display_list_comma_separated(&self.order_by, f)?;
        }
        if let Some(window_frame) = &self.window_frame {
            write!(f, "{}", window_frame)?;
        }
        f.write_str(")")?;
        Ok(())
    }
}
