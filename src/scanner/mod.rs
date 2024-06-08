use lazy_static::lazy_static;
use std::{collections::HashMap, iter::Peekable, str::Chars};

lazy_static! {
    static ref IDENTMAP: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fun", TokenType::Fun);
        m.insert("var", TokenType::Var);
        m.insert("true", TokenType::True);
        m.insert("false", TokenType::False);
        m.insert("return", TokenType::Return);
        m.insert("if", TokenType::If);
        m.insert("else", TokenType::Else);
        m.insert("for", TokenType::For);
        m.insert("nil", TokenType::Nil);
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("while", TokenType::While);
        m
    };
}

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
    eof_sent: bool,
    line: u64,
    // Errors
    // Provide an interface to log errors in the scanning process
}

impl<'a> Scanner<'a> {
    pub fn new(program: &'a str) -> Self {
        Self {
            input: program.chars().peekable(),
            eof_sent: false,
            line: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.input.peek() {
            match c {
                '\n' => {
                    self.read_char();
                    self.line += 1;
                }
                ' ' | '\t' | '\r' => {
                    self.read_char();
                }
                _ => break,
            }
        }
    }

    #[inline]
    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn read_number(&mut self, first: char) -> Result<String, String> {
        let mut number = first.to_string();
        let mut decimal_found = false;

        while let Some(c) = self.input.next() {
            match c {
                v if v.is_ascii_digit() => number.push(c),

                '.' if !decimal_found => {
                    number.push(c);
                    decimal_found = true;

                    if let Some(&next_char) = self.input.peek() {
                        if !next_char.is_ascii_digit() {
                            return Err("trailing dot when parsing number".to_string());
                        }
                    }
                }
                ' ' | '\t' | '\r' | '\n' => return Ok(number),
                v => {
                    return Err(format!(
                        "error in parsing number, unexpected character: {:?}",
                        v
                    ))
                }
            }
        }

        Ok(number)
    }

    fn read_string(&mut self) -> Result<String, String> {
        let mut out = String::new();

        while let Some(c) = self.read_char() {
            match c {
                '"' => return Ok(out),
                '\n' => return Err("unterminated string".to_string()),
                '\\' => {
                    let next_char = self
                        .read_char()
                        .ok_or_else(|| "Unterminated escape sequence".to_string())?;

                    match next_char {
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        '"' => out.push('\"'),
                        '\\' => out.push('\\'),
                        _ => return Err("invalid escape sequence".to_string()),
                    }
                }
                _ => out.push(c),
            }
        }

        Ok(out)
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = first.to_string();
        while self.input.peek().map_or(false, |&c| is_letter(c)) {
            ident.push(self.read_char().unwrap());
        }
        ident
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let ch = self.read_char();

        match ch {
            Some('(') => Some(Token::new(TokenType::LeftParen)),
            Some(')') => Some(Token::new(TokenType::RightParen)),
            Some('{') => Some(Token::new(TokenType::LeftBrace)),
            Some('}') => Some(Token::new(TokenType::RightBrace)),
            Some(',') => Some(Token::new(TokenType::Comma)),
            Some('.') => {
                match self.input.peek() {
                    Some(v) if v.is_ascii_digit() => {
                        // TODO: Log trailing dot error
                        None
                    }

                    _ => Some(Token::new(TokenType::Dot)),
                }
            }
            Some('-') => Some(Token::new(TokenType::Minus)),
            Some('+') => Some(Token::new(TokenType::Plus)),
            Some(';') => Some(Token::new(TokenType::Semicolon)),
            Some('*') => Some(Token::new(TokenType::Star)),
            Some('=') => {
                if let Some(&next) = self.input.peek() {
                    if next == '=' {
                        self.read_char();
                        Some(Token::new(TokenType::EqualEqual))
                    } else {
                        Some(Token::new(TokenType::Equal))
                    }
                } else {
                    Some(Token::new(TokenType::Equal))
                }
            }
            Some('!') => {
                if let Some(&next) = self.input.peek() {
                    if next == '=' {
                        self.read_char();
                        Some(Token::new(TokenType::BangEqual))
                    } else {
                        Some(Token::new(TokenType::Bang))
                    }
                } else {
                    Some(Token::new(TokenType::Bang))
                }
            }
            Some('<') => {
                if let Some(&next) = self.input.peek() {
                    if next == '=' {
                        self.read_char();
                        Some(Token::new(TokenType::LessEqual))
                    } else {
                        Some(Token::new(TokenType::Less))
                    }
                } else {
                    Some(Token::new(TokenType::Less))
                }
            }
            Some('>') => {
                if let Some(&next) = self.input.peek() {
                    if next == '=' {
                        self.read_char();
                        Some(Token::new(TokenType::GreaterEqual))
                    } else {
                        Some(Token::new(TokenType::Greater))
                    }
                } else {
                    Some(Token::new(TokenType::Greater))
                }
            }
            Some('/') => {
                // TODO: All this needs to be cleaned
                if let Some(&next) = self.input.peek() {
                    if next == '/' {
                        // Found a comment!
                        // Skip till the end of line

                        while let Some(next) = self.read_char() {
                            if next == '\n' {
                                break;
                            }
                        }
                        None
                    } else {
                        Some(Token::new(TokenType::Slash))
                    }
                } else {
                    Some(Token::new(TokenType::Slash))
                }
            }
            Some('"') => {
                match self.read_string() {
                    Ok(s) => Some(Token::with_lexeme(TokenType::LString, s)),
                    Err(e) => {
                        // TODO: Log errors
                        return None;
                    }
                }
            }
            Some(c) if c.is_ascii_digit() => {
                match self.read_number(c) {
                    Ok(v) => Some(Token::with_lexeme(TokenType::Number, v)),
                    Err(e) => {
                        // TODO: Log error
                        None
                    }
                }
            }
            Some(c) if is_letter(c) => {
                let ident = self.read_identifier(c);
                Some(lookup_ident(&ident))
            }
            Some('\n') => {
                unreachable!()
            }
            None if !self.eof_sent => {
                self.eof_sent = true;
                Some(Token::new(TokenType::Eof))
            }

            None => None,
            _ => Some(Token::new(TokenType::Illegal)),
        }
    }
}

fn lookup_ident(ident: &str) -> Token {
    match IDENTMAP.get(&ident) {
        Some(v) => Token::new(*v),
        None => Token::with_lexeme(TokenType::Identifier, ident.to_string()),
    }
}

#[inline]
fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

pub struct ScannerError {
    pub line: u64,
    pub message: String,
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
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
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    LString,
    Number,
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
    Eof,
    Illegal,
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    line: u64,
    lexeme: String,
    literal: Option<String>,
}

impl Token {
    pub fn new(ttype: TokenType) -> Self {
        Token {
            ttype,
            line: 0,
            lexeme: "".to_string(),
            literal: None,
        }
    }
    pub fn with_lexeme(ttype: TokenType, l: String) -> Self {
        Token {
            ttype,
            line: 0,
            lexeme: l,
            literal: None,
        }
    }
}
