const KEYWORDS: &[&str] = &["CHIP", "IN", "OUT", "PARTS"];
const SYMBOLS: &[char] = &['=', '{', ';', '}', ':', '(', ')', '[', ']'];

fn is_keyword(s: &str) -> bool {
    KEYWORDS.iter().any(|k| k == &s)
}

fn is_symbol(s: &str) -> bool {
    if s.len() != 1 {
        return false;
    }
    let ch = s.chars().next().unwrap();
    SYMBOLS.iter().any(|symbol| symbol == &ch)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Symbol(char),
    Identifier(String),
    Number(i32),
}

#[derive(Debug, PartialEq)]
pub struct UnexpectedToken {
    expected: Token,
    nline: i32,
    nchar: i32,
}

pub struct Tokeniser<'a> {
    tokens: Vec<Token>,
    itr: std::str::Chars<'a>,
    nline: i32,
    nchar: i32,
}

impl<'a> Tokeniser<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            itr: contents.chars(),
            nline: 1,
            nchar: 1,
        }
    }
    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.itr.next() {
            if ch == '\n' {
                self.nline += 1;
                self.nchar = 1;
            } else {
                self.nchar += 1;
            }
            Some(ch)
        } else {
            None
        }
    }
    fn peek(&mut self) -> Option<char> {
        let mut iter = self.itr.clone();
        iter.next()
    }
    fn next_word(
        &mut self,
        first: fn(char) -> bool,
        split: fn(char) -> bool,
    ) -> Option<&str> {
        let ch = self.peek()?;
        if !first(ch) {
            return None;
        }
        let w = self.itr.as_str().split(split).next()?;
        for _ in w.chars() {
            self.next();
        }
        Some(w)
    }

    pub fn tokenise_chip(&mut self) -> Result<(), UnexpectedToken> {
        self.tokenise_keyword("CHIP")?;
        self.tokenise_identifier()?;
        self.tokenise_symbol('{')?;
        self.tokenise_keyword("IN")?;
        self.tokenise_identifier_list()?;
        self.tokenise_symbol(';')?;
        self.tokenise_keyword("OUT")?;
        self.tokenise_identifier_list()?;
        self.tokenise_symbol(';')?;
        self.tokenise_keyword("PARTS")?;
        self.tokenise_symbol(':')?;
        self.tokenise_parts_list()?;
        self.tokenise_symbol('}')?;
        Ok(())
    }
    pub fn tokenise_parts_list(&mut self) -> Result<(), UnexpectedToken> {
        while let Some(ch) = self.peek() {
            if ch != '}' {
                self.tokenise_part()?;
            } else {
                break;
            }
        }
        Ok(())
    }
    pub fn tokenise_part(&mut self) -> Result<(), UnexpectedToken> {
        self.tokenise_identifier()?;
        self.tokenise_symbol('(')?;
        self.tokenise_assignment_list()?;
        self.tokenise_symbol(')')?;
        self.tokenise_symbol(';')?;
        Ok(())
    }
    pub fn tokenise_assignment_list(&mut self) -> Result<(), UnexpectedToken> {
        while let Some(ch) = self.peek() {
            if ch == ',' {
                self.next();
            } else if ch == ')' {
                break;
            }
            self.tokenise_assignment()?;
        }
        Ok(())
    }
    pub fn tokenise_assignment(&mut self) -> Result<(), UnexpectedToken> {
        self.tokenise_identifier()?;
        self.tokenise_symbol('=')?;
        self.tokenise_identifier()?;
        Ok(())
    }
    pub fn tokenise_identifier_list(&mut self) -> Result<(), UnexpectedToken> {
        while let Some(ch) = self.peek() {
            if ch == ',' {
                self.next();
            } else if ch == ';' {
                break;
            }
            self.tokenise_identifier()?;
        }
        Ok(())
    }
    /// If the current character is a letter or `_`, all following
    /// alphanumeric characters and `_` are considered to be an identifier.
    pub fn tokenise_identifier(&mut self) -> Result<(), UnexpectedToken> {
        self.skip_nontokens();
        let err = Err(UnexpectedToken {
            expected: Token::Identifier("identifier".to_string()),
            nline: self.nline,
            nchar: self.nchar,
        });
        let identifier = self.next_word(
            |first| first.is_alphabetic() || first == '_',
            |ch| !ch.is_alphanumeric() && ch != '_',
        );
        if identifier.is_none() {
            return err;
        }
        let identifier = identifier.unwrap();

        if is_keyword(identifier) || is_symbol(identifier) {
            return err;
        }

        let identifier = identifier.to_string();
        self.tokens.push(Token::Identifier(identifier));

        if let Some(ch) = self.peek() {
            if ch == '[' {
                self.tokenise_symbol('[')?;
                self.tokenise_number()?;
                self.tokenise_symbol(']')?;
            }
        }
        self.skip_nontokens();
        Ok(())
    }
    fn tokenise_number(&mut self) -> Result<(), UnexpectedToken> {
        self.skip_nontokens();
        let err = Err(UnexpectedToken {
            expected: Token::Number(69),
            nline: self.nline,
            nchar: self.nchar,
        });

        let number =
            self.next_word(|first| first.is_digit(10), |ch| !ch.is_digit(10));
        if number.is_none() {
            return err;
        }
        let number = number.unwrap().parse().unwrap();
        self.tokens.push(Token::Number(number));
        self.skip_nontokens();
        Ok(())
    }
    pub fn tokenise_keyword(
        &mut self,
        keyword: &str,
    ) -> Result<(), UnexpectedToken> {
        self.skip_nontokens();
        let token = Token::Keyword(keyword.to_string());
        if self.itr.as_str().starts_with(keyword) {
            for _ in keyword.chars() {
                self.next();
            }
            self.tokens.push(token);
            self.skip_nontokens();
            return Ok(());
        }
        Err(UnexpectedToken {
            expected: token,
            nline: self.nline,
            nchar: self.nchar,
        })
    }
    pub fn tokenise_symbol(
        &mut self,
        symbol: char,
    ) -> Result<(), UnexpectedToken> {
        self.skip_nontokens();
        let token = Token::Symbol(symbol);
        if let Some(ch) = self.next() {
            if ch == symbol {
                self.tokens.push(token);
                self.skip_nontokens();
                return Ok(());
            }
        }
        Err(UnexpectedToken {
            expected: token,
            nline: self.nline,
            nchar: self.nchar,
        })
    }
    fn skip_nontokens(&mut self) {
        while self.skip_whitespace()
            || self.skip_comment("/*", "*/")
            || self.skip_comment("//", "\n")
        {}
    }
    /// Returns `true` if moved the iterator, `false` otherwise.
    fn skip_whitespace(&mut self) -> bool {
        let itr = self.itr.clone();
        let mut moved = false;
        for ch in itr {
            if ch.is_whitespace() {
                self.next();
                moved = true;
                continue;
            }
            break;
        }
        moved
    }
    /// Returns `true` if moved the iterator, `false` otherwise.
    fn skip_comment(&mut self, start: &str, end: &str) -> bool {
        let mut moved = false;
        while self.itr.as_str().starts_with(start) {
            moved = true;
            for _ in start.chars() {
                self.next();
            }
            loop {
                if self.itr.as_str().starts_with(end) {
                    for _ in end.chars() {
                        self.next();
                    }
                    break;
                }
                self.next();
            }
        }
        moved
    }
}

// ============================================================================

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
        let contents = "/*com*/This is/* comment */a string/* comment 2*//**/
// this is a line comment here
// another line comment
//
extra";
        let contents_nc = "This isa string\nextra";
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
    fn tokenise_keyword() {
        let contents = "  CHIP {";
        let mut tokeniser = Tokeniser::new(contents);
        tokeniser.tokenise_keyword("CHIP").unwrap();
        let chip_got = tokeniser.tokens;
        let chip_expected = vec![Token::Keyword("CHIP".to_string())];
        assert_eq!(chip_got, chip_expected);
        assert_eq!(Some('{'), tokeniser.itr.next());
        let contents = "NOTCHIP {";
        let mut tokeniser = Tokeniser::new(contents);
        let chip_err = tokeniser.tokenise_keyword("CHIP").unwrap_err();
        let chip_err_exp = UnexpectedToken {
            expected: Token::Keyword("CHIP".to_string()),
            nline: 1,
            nchar: 1,
        };
        assert_eq!(chip_err, chip_err_exp);
    }
    #[test]
    fn nline_nchar() {
        let contents = "\t\t  CHIP And
// comment
/*comment*/a=b";
        let mut tokeniser = Tokeniser::new(contents);
        assert_eq!((1, 1), (tokeniser.nline, tokeniser.nchar));
        tokeniser.tokenise_keyword("CHIP").unwrap();
        assert_eq!((1, 10), (tokeniser.nline, tokeniser.nchar));
        tokeniser.tokenise_identifier().unwrap();
        assert_eq!((3, 12), (tokeniser.nline, tokeniser.nchar));
    }
    #[test]
    fn tokenise_number() {
        let mut tokeniser = Tokeniser::new("/**/  123");
        let token_exp = Token::Number(123);
        tokeniser.tokenise_number().unwrap();
        assert_eq!(token_exp, tokeniser.tokens[0]);
    }
    #[test]
    fn tokenise_identifier() {
        let token_exp = Token::Identifier("And".to_string());
        let err_exp = UnexpectedToken {
            expected: Token::Identifier("identifier".to_string()),
            nline: 1,
            nchar: 1,
        };

        let mut tokeniser = Tokeniser::new("/**/  And");
        tokeniser.tokenise_identifier().unwrap();
        assert_eq!(token_exp, tokeniser.tokens[0]);

        let mut tokeniser = Tokeniser::new("1And");
        assert_eq!(err_exp, tokeniser.tokenise_identifier().unwrap_err());

        let mut tokeniser = Tokeniser::new("");
        assert_eq!(err_exp, tokeniser.tokenise_identifier().unwrap_err());

        let mut tokeniser = Tokeniser::new("{");
        assert_eq!(err_exp, tokeniser.tokenise_identifier().unwrap_err());

        let mut tokeniser = Tokeniser::new("CHIP");
        assert_eq!(err_exp, tokeniser.tokenise_identifier().unwrap_err());

        let mut tokeniser = Tokeniser::new("a[16]");
        let tokens_exp = vec![
            Token::Identifier("a".to_string()),
            Token::Symbol('['),
            Token::Number(16),
            Token::Symbol(']'),
        ];
        tokeniser.tokenise_identifier().unwrap();
        assert_eq!(tokens_exp, tokeniser.tokens);
    }
    #[test]
    fn tokenise_identifier_list() {
        let mut tokeniser = Tokeniser::new("  a, b  ,c,d/**/,e  /*cc*/;  ");
        let exp_vec = vec![
            Token::Identifier("a".to_string()),
            Token::Identifier("b".to_string()),
            Token::Identifier("c".to_string()),
            Token::Identifier("d".to_string()),
            Token::Identifier("e".to_string()),
        ];
        tokeniser.tokenise_identifier_list().unwrap();
        assert_eq!(exp_vec, tokeniser.tokens);
        assert_eq!(';', tokeniser.next().unwrap());
    }
    #[test]
    fn tokenise_assignment() {
        let mut tokeniser = Tokeniser::new("  a  =  b  ");
        tokeniser.tokenise_assignment().unwrap();
        let tokens = tokeniser.tokens;

        let tokens_exp = vec![
            Token::Identifier("a".to_string()),
            Token::Symbol('='),
            Token::Identifier("b".to_string()),
        ];
        assert_eq!(tokens, tokens_exp);
    }
    #[test]
    fn tokenise_assignment_list() {
        let mut tokeniser = Tokeniser::new(" a = b , c = d , e = f ");
        tokeniser.tokenise_assignment_list().unwrap();
        let tokens = tokeniser.tokens;

        let tokens_exp = vec![
            Token::Identifier("a".to_string()),
            Token::Symbol('='),
            Token::Identifier("b".to_string()),
            Token::Identifier("c".to_string()),
            Token::Symbol('='),
            Token::Identifier("d".to_string()),
            Token::Identifier("e".to_string()),
            Token::Symbol('='),
            Token::Identifier("f".to_string()),
        ];
        assert_eq!(tokens, tokens_exp);
    }
    #[test]
    fn tokenise_part() {
        let mut tokeniser = Tokeniser::new(" Nand( a = a, b = b, out = c ); ");
        tokeniser.tokenise_part().unwrap();
        let tokens_exp = vec![
            Token::Identifier("Nand".to_string()),
            Token::Symbol('('),
            Token::Identifier("a".to_string()),
            Token::Symbol('='),
            Token::Identifier("a".to_string()),
            Token::Identifier("b".to_string()),
            Token::Symbol('='),
            Token::Identifier("b".to_string()),
            Token::Identifier("out".to_string()),
            Token::Symbol('='),
            Token::Identifier("c".to_string()),
            Token::Symbol(')'),
            Token::Symbol(';'),
        ];
        assert_eq!(tokeniser.tokens, tokens_exp);
    }
    #[test]
    fn tokenise_parts_list() {
        let mut tokeniser = Tokeniser::new(
            " Nand( a = a, b = b, out = c ); Nand(a=c, b=c, out=out);",
        );
        tokeniser.tokenise_parts_list().unwrap();
        let tokens_obtained = tokeniser.tokens;

        let mut tokeniser = Tokeniser::new(" Nand( a = a, b = b, out = c );");
        tokeniser.tokenise_part().unwrap();
        let mut tokens_expected = tokeniser.tokens;

        let mut tokeniser = Tokeniser::new(" Nand(a=c, b=c, out=out);");
        tokeniser.tokenise_part().unwrap();
        tokens_expected.append(&mut tokeniser.tokens);

        assert_eq!(tokens_obtained, tokens_expected);
    }
}
