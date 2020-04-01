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
        let mut tokeniser = Tokeniser::new(contents);
        let tokens = tokeniser.tokenise_chip();
        Self { tokens }
    }
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

struct Tokeniser<'a> {
    itr: std::str::Chars<'a>,
}

impl<'a> Tokeniser<'a> {
    fn new(contents: &'a str) -> Self {
        Self {
            itr: contents.chars(),
        }
    }
    fn tokenise_chip(&mut self) -> Vec<Token> {
        let tokens = Vec::new();
        while let Some(ch) = self.itr.next() {
            print!("{:?} ", ch);
        }
        tokens
    }
}
