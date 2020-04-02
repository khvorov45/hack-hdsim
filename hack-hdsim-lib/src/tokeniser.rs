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
        loop {
            self.skip_nontokens();
            match self.itr.next() {
                Some(ch) => print!("{:?}", ch),
                None => break,
            }
        }
        tokens
    }
    fn skip_nontokens(&mut self) {
        self.skip_whitespace();
        self.skip_comment("/*", "*/");
        self.skip_comment("//", "\n");
    }
    /// If the current character is a whitespace, moves the iterator to the next
    /// non-whitespace character.
    fn skip_whitespace(&mut self) {
        let mut itr = self.itr.clone().peekable();
        while let Some(ch) = itr.peek() {
            if ch.is_whitespace() {
                itr.next();
                self.itr.next();
                continue;
            }
            break;
        }
    }
    /// If the current character starts a comment, advances the iterator to
    /// the next non-comment character (handles back-to-back comments as well)
    fn skip_comment(&mut self, start: &str, end: &str) {
        while self.itr.as_str().starts_with(start) {
            for _ in start.chars() {
                self.itr.next();
            }
            loop {
                if self.itr.as_str().starts_with(end) {
                    for _ in end.chars() {
                        self.itr.next();
                    }
                    break;
                }
                self.itr.next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn whitespace_skips() {
        let contents = "  This is \t a string\nwith whitespaces  ";
        let contents_nws = "Thisisastringwithwhitespaces";
        let mut tokeniser = Tokeniser::new(contents);
        let mut no_whitespace = Vec::new();
        loop {
            tokeniser.skip_whitespace();
            match tokeniser.itr.next() {
                Some(ch) => no_whitespace.push(ch),
                None => break,
            }
        }
        let contents_nws_vec: Vec<char> = contents_nws.chars().collect();
        assert_eq!(no_whitespace, contents_nws_vec);
    }
    #[test]
    fn comment_skips() {
        let contents = "/*com*/Thisis/* comment */astring/* comment 2*//**/
// this is a line comment here
extra";
        let contents_nc = "Thisisastring\nextra";
        let mut tokeniser = Tokeniser::new(contents);
        let mut no_com = Vec::new();
        loop {
            tokeniser.skip_comment("/*", "*/");
            tokeniser.skip_comment("//", "\n");
            match tokeniser.itr.next() {
                Some(ch) => no_com.push(ch),
                None => break,
            }
        }
        let contents_nc_vec: Vec<char> = contents_nc.chars().collect();
        assert_eq!(no_com, contents_nc_vec);
    }
}
