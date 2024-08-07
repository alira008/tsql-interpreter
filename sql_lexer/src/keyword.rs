use core::fmt;

pub fn lookup_keyword(keyword: &str) -> Option<Keyword> {
    let normalized_keyword = keyword.to_uppercase();
    match ALL_KEYWORDS.binary_search(&normalized_keyword.as_str()) {
        Ok(index) => Some(ALL_KEYWORDS_INDEX[index].clone()),
        Err(_) => None,
    }
}

impl Default for Keyword {
    fn default() -> Self {
        Keyword::SELECT
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(index) = ALL_KEYWORDS_INDEX.binary_search(self) {
            write!(f, "{}", ALL_KEYWORDS[index])?;
        }

        Err(fmt::Error)
    }
}

macro_rules! define_keyword {
    ($ident:ident = $string_keyword:expr) => {
        pub const $ident: &'static str = $string_keyword;
    };
    // This is macro allows you to call define_keyword!(select)
    // then it will call define_keyword!(SELECT = "SELECT")
    ($ident:ident) => {
        define_keyword!($ident = stringify!($ident));
    };
}

macro_rules! define_keywords {
        ($($ident:ident $(= $string_keyword:expr)?),*) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
            #[allow(non_camel_case_types)]
            pub enum Keyword {
                $($ident),*
            }

            // this holds all the enum Keywords in an array
            pub const ALL_KEYWORDS_INDEX: &[Keyword] = &[$(Keyword::$ident),*];

            // this holds all the string versions of the Keywords in an array
            $(define_keyword!($ident $(= $string_keyword)?);)*
            pub const ALL_KEYWORDS: &[&str] = &[$($ident),*];
        };
    }

// this has this be sorted alphabetically manually because we want to use binary search on
// these keywords
define_keywords!(
    ABS,
    ACOS,
    ALL,
    ALTER,
    AND,
    ANY,
    AS,
    ASC,
    ASIN,
    ATAN,
    AUTOINCREMENT,
    AVG,
    BEGIN,
    BETWEEN,
    BIGINT,
    BIT,
    BY,
    CASCADE,
    CASE,
    CAST,
    CEIL,
    CEILING,
    CHAR,
    COLUMN,
    COLUMNS,
    COMMIT,
    COMMITED,
    CONSTRAINT,
    COS,
    COT,
    COUNT,
    CREATE,
    CURRENT,
    DATE,
    DATETIME,
    DAY,
    DAYOFWEEK,
    DAYOFYEAR,
    DECIMAL,
    DECLARE,
    DEGREES,
    DEFAULT,
    DELETE,
    DENSE_RANK,
    DESC,
    DESCRIBE,
    DISTINCT,
    DO,
    DROP,
    ELSE,
    END,
    ENGINE,
    EXEC,
    EXECUTE,
    EXISTS,
    EXP,
    FALSE,
    FETCH,
    FIRST,
    FIRST_VALUE,
    FLOAT,
    FLOOR,
    FOLLOWING,
    FOREIGN,
    FROM,
    FULL,
    FUNCTION,
    GETDATE,
    GROUP,
    HAVING,
    HOUR,
    HOURS,
    IDENTITY,
    IF,
    IN,
    INCREMENT,
    INDEX,
    INNER,
    INSERT,
    INTEGER,
    INTERSECT,
    INT,
    INTO,
    IS,
    JOIN,
    KEY,
    LAG,
    LAST,
    LAST_VALUE,
    LEAD,
    LEFT,
    LIKE,
    LIMIT,
    LOG,
    LOG10,
    MAX,
    MICROSECOND,
    MICROSECONDS,
    MILLISECOND,
    MILLISECONDS,
    MIN,
    MINUTE,
    MONTH,
    NANOSECOND,
    NANOSECONDS,
    NCHAR,
    NEXT,
    NOT,
    NULL,
    NULLIF,
    NUMERIC,
    NVARCHAR,
    OFFSET,
    ON,
    ONLY,
    OR,
    ORDER,
    OUTER,
    OVER,
    PARTITION,
    PASSWORD,
    PERCENT,
    PI,
    POWER,
    PRECEDING,
    PROCEDURE,
    RADIANS,
    RANDS,
    RANGE,
    RANK,
    REAL,
    RETURN,
    RETURNS,
    REVOKE,
    RIGHT,
    ROLE,
    ROLLBACK,
    ROUND,
    ROW,
    ROWID,
    ROWS,
    ROW_NUMBER,
    SECOND,
    SELECT,
    SET,
    SIGN,
    SIN,
    SMALLINT,
    SNAPSHOT,
    SOME,
    SQRT,
    SQUARE,
    STAGE,
    START,
    STATISTICS,
    STDEV,
    STDEVP,
    SUM,
    TABLE,
    TAN,
    TEMP,
    THEN,
    TIES,
    TIME,
    TINYINT,
    TOP,
    TRANSACTION,
    TRIGGER,
    TRUE,
    TRUNCATE,
    UNBOUNDED,
    UNCOMMITTED,
    UNION,
    UNIQUE,
    UNLOCK,
    UPDATE,
    UPPER,
    USE,
    USER,
    UUID,
    VALUE,
    VALUES,
    VARBINARY,
    VARCHAR,
    VAR,
    VARP,
    WEEK,
    WHEN,
    WHERE,
    WINDOW,
    WITH,
    YEAR
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        let keyword_str = "DESC";
        let keyword = lookup_keyword(keyword_str);

        assert_eq!(Some(Keyword::DESC), keyword)
    }
}
