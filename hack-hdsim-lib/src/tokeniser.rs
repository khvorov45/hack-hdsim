const KEYWORDS: &[&str] = &["CHIP", "IN", "OUT", "PARTS"];
const SYMBOLS: &[&str] = &["=", "{", ";", "}", ":", "(", ")"];

/// Holds the types the tokens may have
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
}

/// Returns `true` if `word` is in `arr`, `false` otherwise
fn contains(arr: &[&str], word: &str) -> bool {
    for arr_word in arr {
        if &word == arr_word {
            return true;
        }
    }
    false
}

/// Holds the representation of a token
#[derive(Debug, PartialEq)]
pub struct Token {
    literal: String,
    token_type: TokenType,
}

impl Token {
    /// Creates a new Token instance.
    /// Note that `literal` is converted into `String`
    pub fn new(literal: &str) -> Self {
        let token_type = {
            if contains(KEYWORDS, literal) {
                TokenType::Keyword
            } else if contains(SYMBOLS, literal) {
                TokenType::Symbol
            } else {
                TokenType::Identifier
            }
        };
        Self {
            literal: String::from(literal),
            token_type,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnexpectedToken {
    expected: Token,
    nline: i32,
    nchar: i32,
}

impl UnexpectedToken {
    /// Creates a new UnexpectedToken instance.
    /// Constructs a Token from `exp_literal` and `exp_type`
    pub fn new(exp_literal: &str, nline: i32, nchar: i32) -> Self {
        Self {
            expected: Token::new(exp_literal),
            nline,
            nchar,
        }
    }
}

/// Tokenises a given string
pub struct Tokeniser<'a> {
    itr: std::str::Chars<'a>,
    nline: i32,
    nchar: i32,
}

impl<'a> Tokeniser<'a> {
    /// Creates a new Tokeniser
    /// Creates a character iterator over `contents` and sets line and character
    /// counters to 1.
    pub fn new(contents: &'a str) -> Self {
        Self {
            itr: contents.chars(),
            nline: 1,
            nchar: 1,
        }
    }
    /// Reads the next character and increments counters.
    /// Returns Some(ch) when there is a character to read. None otherwise.
    fn next_char(&mut self) -> Option<char> {
        if let Some(ch) = self.itr.next() {
            self.advance_counters(ch);
            Some(ch)
        } else {
            None
        }
    }
    /// Returns the next character without advancing the iterator or the
    /// counters
    fn next_char_peek(&mut self) -> Option<char> {
        let mut iter = self.itr.clone();
        iter.next()
    }
    /// Takes a character that was just consumed. If newline, increments line
    /// count and resets char count. Increments char count otherwise.
    fn advance_counters(&mut self, ch: char) {
        if ch == '\n' {
            self.nline += 1;
            self.nchar = 1;
        } else {
            self.nchar += 1;
        }
    }
    /// Tokenises the entire chip description
    pub fn tokenise_chip(&mut self) -> Result<Vec<Token>, UnexpectedToken> {
        self.skip_nontokens();
        let mut tokens = Vec::new();
        tokens.push(self.tokenise_expected("CHIP")?);
        tokens.push(self.tokenise_identifier()?);
        tokens.push(self.tokenise_expected("{")?);
        tokens.push(self.tokenise_expected("IN")?);
        tokens.append(&mut self.tokenise_identifier_list()?);
        tokens.push(self.tokenise_expected(";")?);
        tokens.push(self.tokenise_expected("OUT")?);
        tokens.append(&mut self.tokenise_identifier_list()?);
        tokens.push(self.tokenise_expected(";")?);
        tokens.push(self.tokenise_expected("PARTS")?);
        tokens.push(self.tokenise_expected(":")?);
        tokens.append(&mut self.tokenise_parts_list()?);
        tokens.push(self.tokenise_expected("}")?);
        Ok(tokens)
    }
    /// Tokenises consecutive part declarations
    pub fn tokenise_parts_list(
        &mut self,
    ) -> Result<Vec<Token>, UnexpectedToken> {
        let mut tokens = Vec::new();
        tokens.append(&mut self.tokenise_part()?);
        self.skip_nontokens();
        while let Some(ch) = self.next_char_peek() {
            if ch != '}' {
                tokens.append(&mut self.tokenise_part()?);
                self.skip_nontokens();
            } else {
                break;
            }
        }
        Ok(tokens)
    }
    /// Tokenises a part of the form PartName(pin1=pin2, ...);
    pub fn tokenise_part(&mut self) -> Result<Vec<Token>, UnexpectedToken> {
        let mut tokens = Vec::new();
        tokens.push(self.tokenise_identifier()?);
        tokens.push(self.tokenise_expected("(")?);
        tokens.append(&mut self.tokenise_assignment_list()?);
        tokens.push(self.tokenise_expected(")")?);
        tokens.push(self.tokenise_expected(";")?);
        Ok(tokens)
    }
    /// Returns a vector of tokens of assignments
    pub fn tokenise_assignment_list(
        &mut self,
    ) -> Result<Vec<Token>, UnexpectedToken> {
        let mut tokens = Vec::new();
        tokens.append(&mut self.tokenise_assignment()?);
        self.skip_nontokens();
        while let Some(ch) = self.next_char_peek() {
            if ch == ',' {
                self.next_char();
                tokens.append(&mut self.tokenise_assignment()?);
                self.skip_nontokens();
            } else {
                break;
            }
        }
        Ok(tokens)
    }
    /// Tokenises assignment of the form a=b
    pub fn tokenise_assignment(
        &mut self,
    ) -> Result<Vec<Token>, UnexpectedToken> {
        let mut tokens = Vec::new();
        tokens.push(self.tokenise_identifier()?);
        tokens.push(self.tokenise_expected("=")?);
        tokens.push(self.tokenise_identifier()?);
        Ok(tokens)
    }
    /// Returns a vector of tokens of identifiers
    pub fn tokenise_identifier_list(
        &mut self,
    ) -> Result<Vec<Token>, UnexpectedToken> {
        let mut tokens = Vec::new();
        tokens.push(self.tokenise_identifier()?);
        self.skip_nontokens();
        while let Some(ch) = self.next_char_peek() {
            if ch == ',' {
                self.next_char();
                tokens.push(self.tokenise_identifier()?);
                self.skip_nontokens();
            } else {
                break;
            }
        }
        Ok(tokens)
    }
    /// If the current character is not a digit, all the characters up to the
    /// next whitespace are considered to be an identifier. Returns token with
    /// `literal` of that identifier and `token_type` `Identifier`.
    /// Error otherwise.
    pub fn tokenise_identifier(&mut self) -> Result<Token, UnexpectedToken> {
        self.skip_nontokens();
        let err =
            Err(UnexpectedToken::new("identifier", self.nchar, self.nline));
        let mut itr_char = self.itr.clone();
        let next_ch = itr_char.next();
        if next_ch == None {
            return err;
        }
        let next_ch = next_ch.unwrap();
        if !next_ch.is_alphabetic() && next_ch != '_' {
            return err;
        }
        let mut itr_word =
            self.itr.as_str().split(|ch: char| !ch.is_alphanumeric());
        let iden = itr_word.next().unwrap();
        if contains(KEYWORDS, iden) || contains(SYMBOLS, iden) {
            return err;
        }
        for _ in iden.chars() {
            self.next_char();
        }
        Ok(Token {
            literal: String::from(iden),
            token_type: TokenType::Identifier,
        })
    }
    /// If the current character starts `expct`,
    /// returns Token with literal `expct` and type `tpe`,
    /// and advances the iterator just past `expct`
    /// Error if the current character does not start `expct`
    pub fn tokenise_expected(
        &mut self,
        expct: &str,
    ) -> Result<Token, UnexpectedToken> {
        self.skip_nontokens();
        let expected_token = Token::new(expct);
        if self.itr.as_str().starts_with(expct) {
            for _ in expct.chars() {
                self.next_char();
            }
            return Ok(expected_token);
        }
        Err(UnexpectedToken::new(expct, self.nline, self.nchar))
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
                self.next_char();
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
                self.next_char();
            }
            loop {
                if self.itr.as_str().starts_with(end) {
                    for _ in end.chars() {
                        self.next_char();
                    }
                    break;
                }
                self.next_char();
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
        let contents = "  CHIP {";
        let mut tokeniser = Tokeniser::new(contents);
        let chip_expct = tokeniser.tokenise_expected("CHIP").unwrap();
        let chip_expct_exp = Token::new("CHIP");
        assert_eq!(chip_expct, chip_expct_exp);
        assert_eq!(Some(' '), tokeniser.itr.next());
        assert_eq!(Some('{'), tokeniser.itr.next());
        let contents = "NOTCHIP {";
        let mut tokeniser = Tokeniser::new(contents);
        let chip_err = tokeniser.tokenise_expected("CHIP").unwrap_err();
        let chip_err_exp = UnexpectedToken::new("CHIP", 1, 1);
        assert_eq!(chip_err, chip_err_exp);
    }
    #[test]
    fn nline_nchar() {
        let contents = "\t\t  CHIP And
// comment
/*comment*/a=b";
        let mut tokeniser = Tokeniser::new(contents);
        assert_eq!((1, 1), (tokeniser.nline, tokeniser.nchar));
        tokeniser.tokenise_expected("CHIP").unwrap();
        assert_eq!((1, 9), (tokeniser.nline, tokeniser.nchar));
        tokeniser.tokenise_identifier().unwrap();
        assert_eq!((1, 13), (tokeniser.nline, tokeniser.nchar));
        tokeniser.skip_nontokens();
        assert_eq!((3, 12), (tokeniser.nline, tokeniser.nchar));
    }
    #[test]
    fn tokenise_identifier() {
        let mut tokeniser = Tokeniser::new("/**/  And");
        let token_exp = Token::new("And");
        let err_exp = UnexpectedToken::new("identifier", 1, 1);
        assert_eq!(token_exp, tokeniser.tokenise_identifier().unwrap());
        let mut tokeniser = Tokeniser::new("1And");
        assert_eq!(err_exp, tokeniser.tokenise_identifier().unwrap_err());
        let mut tokeniser = Tokeniser::new("");
        assert_eq!(err_exp, tokeniser.tokenise_identifier().unwrap_err());
        let mut tokeniser = Tokeniser::new("{");
        assert_eq!(err_exp, tokeniser.tokenise_identifier().unwrap_err());
        let mut tokeniser = Tokeniser::new("CHIP");
        assert_eq!(err_exp, tokeniser.tokenise_identifier().unwrap_err());
    }
    #[test]
    fn tokenise_identifier_list() {
        let mut tokeniser = Tokeniser::new("  a, b  ,c,d/**/,e  /*cc*/;  ");
        let exp_vec = vec![
            Token::new("a"),
            Token::new("b"),
            Token::new("c"),
            Token::new("d"),
            Token::new("e"),
        ];
        assert_eq!(exp_vec, tokeniser.tokenise_identifier_list().unwrap());
        assert_eq!(';', tokeniser.next_char().unwrap());
    }
    #[test]
    fn token_new() {
        let token = Tokeniser::new("CHIP").tokenise_expected("CHIP").unwrap();
        let token_exp = Token {
            literal: String::from("CHIP"),
            token_type: TokenType::Keyword,
        };
        assert_eq!(token, token_exp);
        let token = Tokeniser::new("{").tokenise_expected("{").unwrap();
        let token_exp = Token {
            literal: String::from("{"),
            token_type: TokenType::Symbol,
        };
        assert_eq!(token, token_exp);
        let token = Tokeniser::new("a").tokenise_identifier().unwrap();
        let token_exp = Token {
            literal: String::from("a"),
            token_type: TokenType::Identifier,
        };
        assert_eq!(token, token_exp);
    }
    #[test]
    fn tokenise_assignment() {
        let tokens =
            Tokeniser::new("  a  =  b  ").tokenise_assignment().unwrap();
        let tokens_exp =
            vec![Token::new("a"), Token::new("="), Token::new("b")];
        assert_eq!(tokens, tokens_exp);
    }
    #[test]
    fn tokenise_assignment_list() {
        let tokens = Tokeniser::new(" a = b , c = d , e = f ")
            .tokenise_assignment_list()
            .unwrap();
        let tokens_exp = vec![
            Token::new("a"),
            Token::new("="),
            Token::new("b"),
            Token::new("c"),
            Token::new("="),
            Token::new("d"),
            Token::new("e"),
            Token::new("="),
            Token::new("f"),
        ];
        assert_eq!(tokens, tokens_exp);
    }
    #[test]
    fn tokenise_part() {
        let tokens = Tokeniser::new(" Nand( a = a, b = b, out = c ); ")
            .tokenise_part()
            .unwrap();
        let tokens_exp = vec![
            Token::new("Nand"),
            Token::new("("),
            Token::new("a"),
            Token::new("="),
            Token::new("a"),
            Token::new("b"),
            Token::new("="),
            Token::new("b"),
            Token::new("out"),
            Token::new("="),
            Token::new("c"),
            Token::new(")"),
            Token::new(";"),
        ];
        assert_eq!(tokens, tokens_exp);
    }
    #[test]
    fn tokenise_parts_list() {
        let tokens = Tokeniser::new(
            " Nand( a = a, b = b, out = c ); Nand(a=c, b=c, out=out);",
        )
        .tokenise_parts_list()
        .unwrap();
        let mut tokens_exp = Tokeniser::new(" Nand( a = a, b = b, out = c );")
            .tokenise_part()
            .unwrap();
        tokens_exp.append(
            &mut Tokeniser::new(" Nand(a=c, b=c, out=out);")
                .tokenise_part()
                .unwrap(),
        );
        assert_eq!(tokens, tokens_exp);
    }
}
