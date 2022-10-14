pub enum TokenKind {
    IDENTIFIER,
    KEYWORD,
    SEPERATOR,
    OPERATOR,
    LITERAL,
    COMMENT,
    UNKNOWN,
}

pub struct Token {
    pub kind: TokenKind,
    pub start: u64,
    pub end: u64,
}
