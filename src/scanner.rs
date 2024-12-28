use crate::tokens::{Token, Tokens};

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
        self.current_index += 1;
        self.source[self.current_index]
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.current_index == self.source.len() - 1 {
            return false;
        }

        if self.source[self.current_index + 1] != expected {
            return false;
        }
        self.current_index += 1;
        true
    }

    fn add_token(&self, token_type: Tokens) -> Token {
        //lol
        let text: String =
            String::from_utf8_lossy(&self.source[self.start + 1..=self.current_index]).to_string();

        Token::new(token_type, text, self.line)
    }

    fn peek(&self) -> u8 {
        self.source[self.current_index]
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.current_index < self.source.len() - 1 {
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
                62 => {
                    let Tk: Option<Tokens> = match self.match_next(61) {
                        true => None,
                        false => Some(Tokens::Slash),
                    };

                    match Tk {
                        Some(token) => {
                            tokens.push(self.add_token(token));
                        }
                        None => {
                            while self.peek() != 10 && self.current_index < self.source.len() - 1
                            {
                                self.advance();
                            }
                        }
                    }
                }
                _ => {
                    // error(self.line, "unexpected char".to_owned());
                }
            }
        }
        tokens.push(self.add_token(Tokens::EOF));
        tokens
    }
}
