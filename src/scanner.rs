use crate::{
    error::{self, error},
    tokens::{Token, Tokens},
};

pub struct Scanner<'a> {
    source: &'a Vec<u8>,
    current_index: usize,
    line: usize,
    start: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(bytes: &'a Vec<u8>) -> Self {
        Scanner {
            source: bytes,
            current_index: 0,
            line: 0,
            start: 0,
        }
    }

    fn advance(&mut self) -> u8 {
        let ch = self.source[self.current_index];
        self.current_index += 1;
        ch
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.current_index == self.source.len() - 1 {
            return false;
        }

        if self.source[self.current_index] != expected {
            return false;
        }
        self.current_index += 1;
        true
    }

    fn add_token(&self, token_type: Tokens) -> Token {
        //lol
        let text: String =
            String::from_utf8_lossy(&self.source[self.start..self.current_index]).to_string();

        Token::new(token_type, text, self.line)
    }

    fn is_at_end_of_source(&self) -> bool {
        self.current_index >= self.source.len() - 1
    }

    fn string_literal(&mut self) -> Token {
        while (self.peek() != 34 && !self.is_at_end_of_source()) {
            if self.peek() == 10 {
                self.line += 1;
            };
            self.advance();
        }

        if (self.current_index > self.source.len() - 1) {
            error(self.line, "unterminated string literal".to_owned());
        }

        self.advance();
        let lexeme: String =
            String::from_utf8_lossy(&self.source[self.start + 1..self.current_index - 1])
                .to_string();

        Token::new(Tokens::String, lexeme, self.line)
    }

    fn number_literal(&mut self) -> Token {
        while (self.peek().is_ascii_digit()) {
            self.advance();
        }

        if (self.peek() == 46 && self.peek_next().is_ascii_digit()) {
            self.advance();
            while (self.peek().is_ascii_digit()) {
                self.advance();
            }
        }
        let lexeme: String =
            String::from_utf8_lossy(&self.source[self.start..self.current_index]).to_string();

        Token::new(Tokens::Number, lexeme, self.line)
    }
    fn peek(&self) -> u8 {
        self.source[self.current_index]
    }

    fn peek_next(&self) -> u8 {
        if self.current_index + 1 >= self.source.len() {
            return 0;
        }

        return self.source[self.current_index];
    }

    fn get_keywords(&self, ident: &str) -> Tokens {
        match ident {
            "and" => Tokens::And,
            "class" => Tokens::Class,
            "else" => Tokens::Else,
            "false" => Tokens::False,
            "for" => Tokens::For,
            "fun" => Tokens::Fun,
            "if" => Tokens::If,
            "nil" => Tokens::Nil,
            "or" => Tokens::Or,
            "print" => Tokens::Print,
            "return" => Tokens::Return,
            "super" => Tokens::Super,
            "this" => Tokens::This,
            "var" => Tokens::Var,
            "true" => Tokens::True,
            "while" => Tokens::While,
            _ => Tokens::IDENT,
        }
    }

    fn keywords(&mut self) -> Token {
        while (self.peek().is_ascii_alphanumeric()) {
            self.advance();
        }

        let ident_value: String =
            String::from_utf8_lossy(&self.source[self.start..self.current_index]).to_string();

        let ident: Tokens = self.get_keywords(&ident_value);

        Token::new(ident, ident_value, self.line)
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.is_at_end_of_source() {
            self.start = self.current_index;
            let next_char = self.advance();

            match next_char {
                40 => {
                    tokens.push(self.add_token(Tokens::LeftParen));
                }
                41 => {
                    tokens.push(self.add_token(Tokens::RightParen));
                }
                123 => {
                    tokens.push(self.add_token(Tokens::LeftBrace));
                }
                125 => {
                    tokens.push(self.add_token(Tokens::RightBrace));
                }
                44 => {
                    tokens.push(self.add_token(Tokens::Comma));
                }
                46 => {
                    tokens.push(self.add_token(Tokens::Dot));
                }
                45 => {
                    tokens.push(self.add_token(Tokens::Minus));
                }
                43 => {
                    tokens.push(self.add_token(Tokens::Plus));
                }
                59 => {
                    tokens.push(self.add_token(Tokens::Semicolon));
                }
                42 => {
                    tokens.push(self.add_token(Tokens::Star));
                }

                32 | 9 | 13 => {}
                10 => {
                    self.line += 1;
                }
                21 => {
                    let Tk = match self.match_next(61) {
                        true => Tokens::BangEq,
                        false => Tokens::Bang,
                    };
                    tokens.push(self.add_token(Tk));
                }
                61 => {
                    let Tk = match self.match_next(61) {
                        true => Tokens::EqEq,
                        false => Tokens::Eq,
                    };
                    tokens.push(self.add_token(Tk));
                }
                60 => {
                    let Tk = match self.match_next(61) {
                        true => Tokens::LtEq,
                        false => Tokens::Lt,
                    };
                    tokens.push(self.add_token(Tk));
                }
                62 => {
                    let Tk = match self.match_next(61) {
                        true => Tokens::GtEq,
                        false => Tokens::Gt,
                    };
                    tokens.push(self.add_token(Tk));
                }
                34 => {
                    tokens.push(self.string_literal());
                }
                47 => {
                    let Tk: Option<Tokens> = match self.match_next(47) {
                        true => None,
                        false => Some(Tokens::Slash),
                    };

                    match Tk {
                        Some(token) => {
                            tokens.push(self.add_token(token));
                        }
                        None => {
                            while self.peek() != 10 && !self.is_at_end_of_source() {
                                self.advance();
                            }
                        }
                    }
                }
                default => {
                    if default.is_ascii_digit() {
                        tokens.push(self.number_literal());
                    } else if default.is_ascii_alphanumeric() {
                        tokens.push(self.keywords())
                    } else {
                        error(self.line, "unexpected char".to_owned());
                    }
                }
            }
        }
        tokens.push(self.add_token(Tokens::EOF));
        tokens
    }
}
