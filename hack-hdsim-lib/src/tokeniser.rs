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
    itr: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Tokeniser<'a> {
    fn new(contents: &'a str) -> Self {
        Self {
            itr: contents.chars().peekable(),
        }
    }
    fn tokenise_chip(&mut self) -> Vec<Token> {
        let tokens = Vec::new();
        while let Some(ch) = self.itr.next() {
            self.skip_nontokens();
            print!("{:?} ", ch);
        }
        tokens
    }
    fn skip_nontokens(&mut self) {
        while let Some(ch) = self.itr.peek() {
            if ch.is_whitespace() {
                self.itr.next();
                continue;
            }
            break;
        }
    }
}
