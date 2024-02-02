use sql_parser::visitor::{walk_query, Visitor};

use crate::settings::{FormatterSettings, KeywordCase};

pub struct Formatter {
    settings: FormatterSettings,
    formatted_query: String,
}

impl Formatter {
    pub fn new(settings: FormatterSettings) -> Self {
        let formatted_query = "".to_string();
        Self {
            settings,
            formatted_query,
        }
    }

    pub fn format(&mut self, input: &str) {
        let lexer = sql_parser::lexer::Lexer::new(input);
        let mut parser = sql_parser::Parser::new(lexer);
        let query = parser.parse();

        // walk the ast
        walk_query(self, &query);
    }

    pub fn formatted_query(&self) -> &str {
        &self.formatted_query
    }

    fn print_keyword(&mut self, keyword: &str) {
        match self.settings.keyword_case {
            KeywordCase::Upper => self.formatted_query.push_str(&keyword.to_uppercase()),
            KeywordCase::Lower => self.formatted_query.push_str(&keyword.to_lowercase()),
        }
    }

    fn print_indent(&mut self) {
        let indent = self.settings.indent_width;
        if self.settings.use_tab {
            for _ in 0..indent {
                self.formatted_query.push_str("\t");
            }
        } else {
            for _ in 0..indent {
                self.formatted_query.push_str(" ");
            }
        }
    }

    fn print_select_column_comma(&mut self) {
        if let Some(indent_comma_lists) = self.settings.indent_comma_lists {
            match indent_comma_lists {
                crate::IndentCommaLists::TrailingComma => {
                    self.formatted_query.push_str(",\n");
                    self.print_indent();
                }
                crate::IndentCommaLists::SpaceAfterComma => {
                    self.formatted_query.push_str("\n");
                    self.print_indent();
                    self.formatted_query.push_str(", ");
                }
            }
        } else {
            self.formatted_query.push_str("\n");
            self.print_indent();
            self.formatted_query.push_str(",");
        }
    }

    fn print_expression_list_comma(&mut self) {
        self.formatted_query.push_str(", ");
    }

    fn print_in_list_comma(&mut self) {
        if self.settings.indent_in_lists {
            self.print_select_column_comma();
        } else {
            self.formatted_query.push_str(", ");
        }
    }
}

impl Visitor for Formatter {
    fn visit_token(&mut self, token: &sql_parser::token::Token) {
        match token.kind() {
            sql_parser::token::Kind::Keyword(_) => match self.settings.keyword_case {
                KeywordCase::Upper => {
                    self.formatted_query
                        .push_str(&token.literal().to_string().to_uppercase());
                }
                KeywordCase::Lower => {
                    self.formatted_query
                        .push_str(&token.literal().to_string().to_lowercase());
                }
            },
            _ => self.formatted_query.push_str(&token.literal().to_string()),
        }
    }

    fn visit_select_query(&mut self, query: &sql_parser::ast::SelectStatement) {
        self.print_keyword("SELECT ");
        self.visit_select_top_argument(&query.top);
        self.visit_select_columns(&query.columns);
        self.visit_select_into_table(&query.into_table);
        self.visit_select_table(&query.table);
        self.visit_select_where_clause(&query.where_clause);
        self.visit_select_group_by(&query.group_by);
        self.visit_select_having(&query.having);
        self.visit_select_order_by(&query.order_by);
        self.visit_select_offset(&query.offset);
        self.visit_select_fetch(&query.fetch);
    }

    fn visit_select_top_argument(&mut self, top: &Option<sql_parser::ast::TopArg>) {
        if let Some(top) = top {
            self.print_keyword("TOP ");
            self.visit_expression(&top.quantity);
            self.formatted_query.push_str(" ");
            if top.percent {
                self.print_keyword("PERCENT ");
            }
            if top.with_ties {
                self.print_keyword("WITH TIES ");
            }
        }
    }

    fn visit_select_columns(&mut self, columns: &[sql_parser::ast::SelectItem]) {
        for (i, column) in columns.iter().enumerate() {
            if i > 0 {
                self.print_select_column_comma();
            }
            self.visit_select_item(column);
        }
        self.formatted_query.push_str("\n");
    }

    fn visit_select_item(&mut self, item: &sql_parser::ast::SelectItem) {
        match item {
            sql_parser::ast::SelectItem::Unnamed(expr) => {
                self.visit_expression(expr);
            }
            sql_parser::ast::SelectItem::WithAlias {
                expression,
                as_token,
                alias,
            } => {
                self.visit_expression(expression);
                if *as_token {
                    self.print_keyword(" AS ");
                }
                self.formatted_query.push_str(alias);
            }
            sql_parser::ast::SelectItem::WildcardWithAlias {
                expression,
                as_token,
                alias,
            } => {
                self.visit_expression(expression);
                if *as_token {
                    self.print_keyword(" AS ");
                }
                self.formatted_query.push_str(alias);
            }
            sql_parser::ast::SelectItem::Wildcard => {
                self.formatted_query.push_str("*");
            }
        }
    }

    fn visit_select_into_table(&mut self, arg: &Option<sql_parser::ast::IntoArg>) {
        if let Some(into_arg) = arg {
            self.print_keyword("INTO ");
            self.visit_expression(&into_arg.table);

            if let Some(file_group) = &into_arg.file_group {
                self.print_keyword("ON ");
                self.visit_expression(file_group);
            }
        }
    }

    fn visit_table_source(&mut self, table: &sql_parser::ast::TableSource) {
        match table {
            sql_parser::ast::TableSource::Table { name, is_as, alias } => {
                self.visit_expression(name);
                if let Some(alias) = alias {
                    self.formatted_query.push_str(" ");
                    if *is_as {
                        self.print_keyword("AS ");
                    }
                    self.formatted_query.push_str(alias);
                }
            }
            sql_parser::ast::TableSource::TableValuedFunction {
                function,
                is_as,
                alias,
            } => {
                self.visit_expression(function);
                if let Some(alias) = alias {
                    self.formatted_query.push_str(" ");
                    if *is_as {
                        self.print_keyword("AS ");
                    }
                    self.formatted_query.push_str(alias);
                }
            }
            _ => unimplemented!(),
        }
    }

    fn visit_select_table(&mut self, arg: &Option<sql_parser::ast::TableArg>) {
        if let Some(table_arg) = arg {
            self.print_keyword("FROM ");
            self.visit_table_source(&table_arg.table);
            self.formatted_query.push_str("\n");
            for (i, join) in table_arg.joins.iter().enumerate() {
                if i == 0 {
                    self.print_keyword("JOIN ");
                }
                self.visit_table_join(join);
            }
            if table_arg.joins.len() > 0 {
                self.formatted_query.push_str("\n");
            }
        } else {
            unreachable!();
        }
    }

    fn visit_table_join(&mut self, join: &sql_parser::ast::Join) {
        self.visit_table_join_type(&join.join_type);
        self.visit_table_source(&join.table);
        self.print_keyword(" ON ");
        if let Some(condition) = &join.condition {
            self.visit_expression(condition);
        }
    }

    fn visit_table_join_type(&mut self, join_type: &sql_parser::ast::JoinType) {
        match join_type {
            sql_parser::ast::JoinType::Inner => self.print_keyword("INNER JOIN "),
            sql_parser::ast::JoinType::Left => self.print_keyword("LEFT JOIN "),
            sql_parser::ast::JoinType::LeftOuter => self.print_keyword("LEFT OUTER JOIN "),
            sql_parser::ast::JoinType::Right => self.print_keyword("RIGHT JOIN "),
            sql_parser::ast::JoinType::RightOuter => self.print_keyword("RIGHT OUTER JOIN "),
            sql_parser::ast::JoinType::FullOuter => self.print_keyword("FULL OUTER JOIN "),
            sql_parser::ast::JoinType::Full => self.print_keyword("FULL JOIN "),
            sql_parser::ast::JoinType::CrossApply => todo!(),
            sql_parser::ast::JoinType::OuterApply => todo!(),
        }
    }

    fn visit_select_where_clause(&mut self, where_clause: &Option<sql_parser::ast::Expression>) {
        if let Some(where_clause) = where_clause {
            self.print_keyword("WHERE ");
            self.visit_expression(where_clause);
            self.formatted_query.push_str("\n");
        }
    }

    fn visit_binary_expression(&mut self, expression: &sql_parser::ast::Expression) {
        if let sql_parser::ast::Expression::Binary {
            left,
            right,
            operator,
        } = expression
        {
            self.visit_expression(left);
            self.formatted_query.push_str(" ");
            if matches!(
                operator.kind(),
                sql_parser::token::Kind::Keyword(sql_parser::keywords::Keyword::AND)
                    | sql_parser::token::Kind::Keyword(sql_parser::keywords::Keyword::OR)
            ) && self.settings.indent_between_conditions
            {
                self.formatted_query.push_str("\n");
                self.print_indent();
            }
            self.visit_token(operator);
            self.formatted_query.push_str(" ");
            self.visit_expression(right);
        } else {
            unreachable!();
        }
    }

    fn visit_select_group_by(&mut self, group_by: &[sql_parser::ast::Expression]) {
        for (i, expression) in group_by.iter().enumerate() {
            if i == 0 {
                self.print_keyword("GROUP BY ");
            }
            if i > 0 {
                self.print_select_column_comma();
            }
            self.visit_expression(expression);
        }
        if group_by.len() > 0 {
            self.formatted_query.push_str("\n");
        }
    }

    fn visit_select_having(&mut self, having_arg: &Option<sql_parser::ast::Expression>) {
        if let Some(having) = having_arg {
            self.print_keyword("HAVING ");
            self.visit_expression(having);
            self.formatted_query.push_str("\n");
        }
    }

    fn visit_select_order_by(&mut self, order_by_args: &[sql_parser::ast::OrderByArg]) {
        for (i, order_by) in order_by_args.iter().enumerate() {
            if i == 0 {
                self.print_keyword("ORDER BY ");
            }
            if i > 0 {
                self.print_select_column_comma();
            }
            self.visit_expression(&order_by.column);
            self.formatted_query.push_str(" ");
            if let Some(asc) = order_by.asc {
                if asc {
                    self.print_keyword("ASC ");
                } else {
                    self.print_keyword("DESC ");
                }
            }
        }
        if order_by_args.len() > 0 {
            self.formatted_query.push_str("\n");
        }
    }

    fn visit_select_offset(&mut self, arg: &Option<sql_parser::ast::OffsetArg>) {
        if let Some(offset) = arg {
            self.print_keyword("OFFSET ");
            self.visit_expression(&offset.value);
            self.formatted_query.push_str(" ");
            self.visit_select_offset_fetch_row_or_rows(offset.row);
            self.formatted_query.push_str("\n");
        }
    }

    fn visit_select_fetch(&mut self, arg: &Option<sql_parser::ast::FetchArg>) {
        if let Some(fetch) = arg {
            self.print_keyword("FETCH ");
            self.visit_select_fetch_next_or_first(fetch.first);
            self.visit_expression(&fetch.value);
            self.formatted_query.push_str(" ");
            self.visit_select_offset_fetch_row_or_rows(fetch.row);
            self.print_keyword("ONLY ");
            self.formatted_query.push_str("\n");
        }
    }

    fn visit_select_offset_fetch_row_or_rows(&mut self, row_or_rows: sql_parser::ast::RowOrRows) {
        match row_or_rows {
            sql_parser::ast::RowOrRows::Row => self.print_keyword("ROW "),
            sql_parser::ast::RowOrRows::Rows => self.print_keyword("ROWS "),
        }
    }
    fn visit_select_fetch_next_or_first(&mut self, next_or_first: sql_parser::ast::NextOrFirst) {
        match next_or_first {
            sql_parser::ast::NextOrFirst::Next => self.print_keyword("NEXT "),
            sql_parser::ast::NextOrFirst::First => self.print_keyword("FIRST "),
        }
    }

    fn visit_is_true_expression(&mut self, expression: &sql_parser::ast::Expression) {
        self.print_keyword("IS ");
        self.visit_expression(expression);
    }
    fn visit_is_not_true_expression(&mut self, expression: &sql_parser::ast::Expression) {
        self.print_keyword("IS NOT ");
        self.visit_expression(expression);
    }
    fn visit_is_null_expression(&mut self, expression: &sql_parser::ast::Expression) {
        self.visit_expression(expression);
        self.print_keyword("IS NULL ");
    }
    fn visit_is_not_null_expression(&mut self, expression: &sql_parser::ast::Expression) {
        self.visit_expression(expression);
        self.print_keyword("IS NOT NULL ");
    }
    fn visit_in_list_expression(&mut self, expression: &sql_parser::ast::Expression) {
        if let sql_parser::ast::Expression::InList {
            expression,
            list,
            not,
        } = expression
        {
            if *not {
                self.print_keyword("NOT ");
            }
            self.print_keyword("IN ");
            self.visit_expression(expression);
            self.formatted_query.push_str("(");
            for (i, expression) in list.iter().enumerate() {
                if i > 0 {
                    self.print_in_list_comma();
                }
                self.visit_expression(expression);
            }
            self.formatted_query.push_str(")");
        }
    }
    fn visit_between_expression(&mut self, expression: &sql_parser::ast::Expression) {
        if let sql_parser::ast::Expression::Between { low, high, not } = expression {
            if *not {
                self.print_keyword("NOT ");
            }
            self.print_keyword("BETWEEN ");
            self.visit_expression(low);
            self.print_keyword(" AND ");
            self.visit_expression(high);
        }
    }
    fn visit_any_expression(&mut self, expression: &sql_parser::ast::Expression) {
        if let sql_parser::ast::Expression::Any {
            left,
            right,
            operator,
        } = expression
        {
            self.visit_expression(left);
            self.formatted_query.push_str(" ");
            self.visit_token(operator);
            self.print_keyword(" ANY ");
            self.formatted_query.push_str("(");
            self.visit_expression(right);
            self.formatted_query.push_str(")");
        }
    }
    fn visit_all_expression(&mut self, expression: &sql_parser::ast::Expression) {
        if let sql_parser::ast::Expression::All {
            left,
            right,
            operator,
        } = expression
        {
            self.visit_expression(left);
            self.formatted_query.push_str(" ");
            self.visit_token(operator);
            self.print_keyword(" ALL ");
            self.formatted_query.push_str("(");
            self.visit_expression(right);
            self.formatted_query.push_str(")");
        }
    }
    fn visit_some_expression(&mut self, expression: &sql_parser::ast::Expression) {
        if let sql_parser::ast::Expression::Some {
            left,
            right,
            operator,
        } = expression
        {
            self.visit_expression(left);
            self.formatted_query.push_str(" ");
            self.visit_token(operator);
            self.print_keyword(" SOME ");
            self.formatted_query.push_str("(");
            self.visit_expression(right);
            self.formatted_query.push_str(")");
        }
    }
    fn visit_exists_expression(&mut self, expression: &sql_parser::ast::Expression) {
        self.print_keyword("EXISTS ");
        self.formatted_query.push_str("(");
        self.visit_expression(expression);
        self.formatted_query.push_str(")");
    }
    fn visit_expression_list_expression(&mut self, expression: &[sql_parser::ast::Expression]) {
        self.formatted_query.push_str("(");
        for (i, expression) in expression.iter().enumerate() {
            if i > 0 {
                self.print_expression_list_comma();
            }
            self.visit_expression(expression);
        }
        self.formatted_query.push_str(")");
    }
    fn visit_function_expression(&mut self, expression: &sql_parser::ast::Expression) {
        if let sql_parser::ast::Expression::Function { name, args, over } = expression {
            self.visit_expression(name);
            self.visit_expression(args);

            if let Some(over) = over {
                self.print_keyword(" OVER");
                self.visit_select_window_over_clause(over);
            }
        }
    }
    fn visit_select_window_over_clause(&mut self, over_clause: &sql_parser::ast::OverClause) {
        self.formatted_query.push_str("(\n");
        self.print_indent();
        self.print_indent();

        for (i, partition_by) in over_clause.partition_by.iter().enumerate() {
            if i == 0 {
                self.print_keyword("PARTITION BY ");
            }
            if i > 0 {
                self.print_expression_list_comma();
            }
            self.visit_expression(partition_by);
        }
        if over_clause.order_by.len() > 0 {
            self.formatted_query.push_str(" ");
        }
        self.visit_select_order_by(&over_clause.order_by);
        if let Some(window_frame) = &over_clause.window_frame {
            self.visit_window_frame(window_frame);
        }

        self.formatted_query.push_str("\n");
        self.print_indent();
        self.print_indent();
        self.formatted_query.push_str(")");
    }
    fn visit_window_frame(&mut self, window_frame: &sql_parser::ast::WindowFrame) {
        if let Some(end) = &window_frame.end {
            self.formatted_query.push_str(" ");
            self.visit_window_frame_rows_or_range(window_frame.rows_or_range);
            self.print_keyword(" BETWEEN ");
            self.visit_window_frame_bound(&window_frame.start);
            self.print_keyword(" AND ");
            self.visit_window_frame_bound(end);
        } else {
            self.formatted_query.push_str(" ");
            self.visit_window_frame_rows_or_range(window_frame.rows_or_range);
            self.formatted_query.push_str(" ");
            self.visit_window_frame_bound(&window_frame.start);
        }
    }
    fn visit_window_frame_rows_or_range(&mut self, _rows_or_range: sql_parser::ast::RowsOrRange) {}
    fn visit_window_frame_bound(&mut self, bound: &sql_parser::ast::WindowFrameBound) {
        match bound {
            sql_parser::ast::WindowFrameBound::Preceding(expression) => {
                self.print_keyword("PRECEDING ");
                self.visit_expression(expression)
            }
            sql_parser::ast::WindowFrameBound::Following(expression) => {
                self.print_keyword("FOLLOWING ");
                self.visit_expression(expression);
            }
            sql_parser::ast::WindowFrameBound::CurrentRow => {
                self.print_keyword("CURRENT ROW");
            }
            sql_parser::ast::WindowFrameBound::UnboundedPreceding => {
                self.print_keyword("UNBOUNDED PRECEDING");
            }
            sql_parser::ast::WindowFrameBound::UnboundedFollowing => {
                self.print_keyword("UNBOUNDED FOLLOWING");
            }
        }
    }
    fn visit_compound_literal(&mut self, tokens: &[sql_parser::token::Token]) {
        for (i, token) in tokens.iter().enumerate() {
            if i > 0 {
                self.formatted_query.push_str(".");
            }
            self.visit_token(token);
        }
    }
    fn visit_cte_statement(&mut self, statement: &sql_parser::ast::Statement) {
        if let sql_parser::ast::Statement::CTE { ctes, statement } = statement {
            self.print_keyword("WITH ");
            for (i, cte) in ctes.iter().enumerate() {
                if i > 0 {
                    self.formatted_query.push_str(",\n");
                }
                self.visit_cte(cte);
            }

            self.visit_select_query(statement);
        } else {
            unreachable!();
        }
    }
    fn visit_cte(&mut self, cte: &sql_parser::ast::CommonTableExpression) {
        self.visit_expression(&cte.name);

        for (i, column) in cte.columns.iter().enumerate() {
            if i == 0 {
                self.formatted_query.push_str("(");
            }
            self.visit_expression(column);
        }
        if cte.columns.len() > 0 {
            self.formatted_query.push_str(")\n");
        } else {
            self.formatted_query.push_str("\n");
        }

        self.print_keyword("AS\n");
        self.formatted_query.push_str("(\n");
        self.visit_statement(&cte.query);
        self.formatted_query.push_str(")\n");
    }
}
