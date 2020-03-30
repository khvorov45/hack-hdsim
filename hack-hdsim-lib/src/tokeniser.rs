const _KEYWORDS: &[&str] = &["CHIP", "IN", "OUT", "PARTS"];

#[derive(Debug)]
struct Token {
    literal: String,
    token_type: TokenType,
}

#[derive(Debug)]
enum TokenType {
    Keyword,
    Unknown,
}

pub fn tokenise_hdl(contents: String) {
    println!("String to tokenise:\n{}", contents);
    // Probably need to remove comments here
    let temp = contents.split(' ');
    println!("Tokens:\n");
    let mut tokens = Vec::new();
    for tok in temp {
        let literal = tok.trim();
        let token_type = match literal {
            "CHIP" => TokenType::Keyword,
            _ => TokenType::Unknown,
        };
        let token = Token {
            literal: String::from(literal),
            token_type,
        };
        println!("{:?}", token);
        tokens.push(token);
    }
}
