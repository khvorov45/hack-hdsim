const _KEYWORDS: &[&str] = &["CHIP", "IN", "OUT", "PARTS"];

#[derive(Debug)]
struct Token {
    literal: String,
    token_type: TokenType,
}

#[derive(Debug)]
enum TokenType {
    _Keyword,
    _Symbol,
    _Identifier,
}

struct _Tokens {
    tokens: Vec<Token>,
}
