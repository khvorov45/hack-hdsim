const _KEYWORDS: &[&str] = &["CHIP", "IN", "OUT", "PARTS"];

#[derive(Debug, PartialEq)]
enum TokenType {
    Keyword,
    Symbol,
    _Identifier,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    literal: String,
    token_type: TokenType,
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

#[derive(Debug, PartialEq)]
struct UnexpectedToken {
    expected: Token,
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

        self.skip_nontokens();
        println!("{:?}", self.tokenise_expected("CHIP", TokenType::Keyword));
        self.skip_nontokens();
        println!("{:?}", self.tokenise_expected("{", TokenType::Symbol));

        tokens
    }
    /// If the current character starts `expct`,
    /// returns Token with literal `expct` and type `tpe`,
    /// and advances the iterator just past `expct`
    /// Error if the current character does not start `expct`
    fn tokenise_expected(
        &mut self,
        expct: &str,
        tpe: TokenType,
    ) -> Result<Token, UnexpectedToken> {
        let expected_token = Token {
            literal: String::from(expct),
            token_type: tpe,
        };
        if self.itr.as_str().starts_with(expct) {
            for _ in expct.chars() {
                self.itr.next();
            }
            return Ok(expected_token);
        }
        Err(UnexpectedToken {
            expected: expected_token,
        })
    }
    /// If the current character is a whitespace or starts a comment,
    /// moves the iterator to the character that starts a token.
    fn skip_nontokens(&mut self) {
        while self.skip_whitespace()
            || self.skip_comment("/*", "*/")
            || self.skip_comment("//", "\n")
        {}
    }
    /// If the current character is a whitespace, moves the iterator to the next
    /// non-whitespace character.
    /// Returns `true` if moved the iterator, `false` otherwise.
    fn skip_whitespace(&mut self) -> bool {
        let mut itr = self.itr.clone();
        let mut moved = false;
        while let Some(ch) = itr.next() {
            if ch.is_whitespace() {
                self.itr.next();
                moved = true;
                continue;
            }
            break;
        }
        moved
    }
    /// If the current character starts a comment, advances the iterator to
    /// the next non-comment character (handles back-to-back comments as well).
    /// Returns `true` if moved the iterator, `false` otherwise.
    fn skip_comment(&mut self, start: &str, end: &str) -> bool {
        let mut moved = false;
        while self.itr.as_str().starts_with(start) {
            moved = true;
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
        moved
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn skip_whitespace() {
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
        let contents = "   a";
        let mut tokeniser = Tokeniser::new(contents);
        assert_eq!(true, tokeniser.skip_whitespace());
        let contents = "a    ";
        let mut tokeniser = Tokeniser::new(contents);
        assert_eq!(false, tokeniser.skip_whitespace());
    }
    #[test]
    fn skip_comment() {
        let contents = "/*com*/Thisis/* comment */astring/* comment 2*//**/
// this is a line comment here
// another line comment
//
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
        let contents = "/**/a";
        let mut tokeniser = Tokeniser::new(contents);
        assert_eq!(true, tokeniser.skip_comment("/*", "*/"));
        let contents = "a/**/";
        let mut tokeniser = Tokeniser::new(contents);
        assert_eq!(false, tokeniser.skip_comment("/*", "*/"));
    }
    #[test]
    fn skip_nontokens() {
        let contents = "  /* comment */  a /* comment */ = b  // comment
// comment
c=d
";
        let contents_tok_only = "a=bc=d";
        let mut tokeniser = Tokeniser::new(contents);
        let mut tok_only = Vec::new();
        loop {
            tokeniser.skip_nontokens();
            match tokeniser.itr.next() {
                Some(ch) => tok_only.push(ch),
                None => break,
            }
        }
        let contents_to_vec: Vec<char> = contents_tok_only.chars().collect();
        assert_eq!(tok_only, contents_to_vec);
    }
    #[test]
    fn tokenise_expected() {
        let contents = "CHIP {";
        let mut tokeniser = Tokeniser::new(contents);
        let chip_expct = tokeniser
            .tokenise_expected("CHIP", TokenType::Keyword)
            .unwrap();
        let chip_expct_exp = Token {
            literal: String::from("CHIP"),
            token_type: TokenType::Keyword,
        };
        assert_eq!(chip_expct, chip_expct_exp);
        assert_eq!(Some(' '), tokeniser.itr.next());
        assert_eq!(Some('{'), tokeniser.itr.next());
        let contents = "NOTCHIP {";
        let mut tokeniser = Tokeniser::new(contents);
        let chip_err = tokeniser
            .tokenise_expected("CHIP", TokenType::Keyword)
            .unwrap_err();
        let chip_err_exp = UnexpectedToken {
            expected: chip_expct_exp,
        };
        assert_eq!(chip_err, chip_err_exp);
    }
}
