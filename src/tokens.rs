#[derive(Debug)]
pub enum Tokens {
    //single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    Eq,
    Gt,
    Lt,
    //multi char tokens
    BangEq,
    GtEq,
    LtEq,
    EqEq,
    //literals
    IDENT,
    String,
    Number,
    //keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    token_type: Tokens,
    lexeme: String,
    line: usize,
    //FIXME: need to add literal
}

impl Token {
    pub fn new(token_type: Tokens, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            "type {:?} lexeme {:?} line {}",
            self.token_type, self.lexeme, self.line
        )
    }
}
