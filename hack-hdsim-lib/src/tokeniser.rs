const _KEYWORDS: &[&str] = &["CHIP", "IN", "OUT", "PARTS"];

#[derive(Debug)]
pub struct Token {
    literal: String,
    token_type: TokenType,
}

#[derive(Debug)]
enum TokenType {
    _Keyword,
    _Symbol,
    _Identifier,
}

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(contents: &str) -> Self {
        println!("Contents:\n{}", contents);
        let tokens = Vec::new();
        Self { tokens }
    }
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
