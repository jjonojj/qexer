// the tokenizer.
#![allow(dead_code)]

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Str(String),
    StrLit(String), // TODO
    Num(String),

    // keywords
    Fn,
    Let,
    Return,

    //
    LParen,
    RParen,
    LBrace,
    RBrace,
    LSqBracket,
    RSqBracket,
    SemiC,
    Colon,
    DoubleColon,
    ExclMark,
    QuestMark,
    Eq,
    Comma,

    //
    Star,
    Slash,
    BSlash,
    Plus,
    Minus,

    //
    Quote,
    Ampersand,
    CommAt,
    Dollar,

    //
    EOF,
    Undef,
    None,
}

// pretty hehe
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Str(val) => write!(f, "string(value: {val})"),
            Token::StrLit(val) => write!(f, "string literal(value: {val})"),
            Token::Num(val) => write!(f, "number(value: {val})"),
            Token::Fn => write!(f, "function"),
            Token::Let => write!(f, "let keyword"),
            Token::Return => write!(f, "return keyword"),
            Token::LParen => write!(f, "left parenthesis"),
            Token::RParen => write!(f, "right parenthesis"),
            Token::LBrace => write!(f, "left brace"),
            Token::RBrace => write!(f, "right brace"),
            Token::LSqBracket => write!(f, "left square bracket"),
            Token::RSqBracket => write!(f, "right square bracket"),
            Token::SemiC => write!(f, "semicolon"),
            Token::Colon => write!(f, "colon"),
            Token::DoubleColon => write!(f, "double colon"),
            Token::ExclMark => write!(f, "exclamation mark"),
            Token::QuestMark => write!(f, "question mark"),
            Token::Eq => write!(f, "equal sign"),
            Token::Comma => write!(f, "comma"),

            Token::Star => write!(f, "star"),
            Token::Slash => write!(f, "slash"),
            Token::BSlash => write!(f, "backslash"),
            Token::Plus => write!(f, "plus"),
            Token::Minus => write!(f, "minus"),

            Token::Quote => write!(f, "single quote"),
            Token::Ampersand => write!(f, "ampersand"),
            Token::CommAt => write!(f, "at symbol"),
            Token::Dollar => write!(f, "dollar sign"),

            Token::EOF => write!(f, "end of file"),
            _ => write!(f, "undefined / none"),
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    src: String,
    idx: usize,
    comment_char: char,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            src: input.to_owned(),
            idx: 0,
            comment_char: ';',
        }
    }

    fn current(&self) -> char {
        match self.src.chars().nth(self.idx) {
            Some(char) => char,
            None => '\0',
        }
    }

    fn advance(&mut self) {
        self.idx += 1;
    }

    fn get_str(&mut self) -> &str {
        let start = self.idx;
        while self.current().is_ascii_alphanumeric() {
            self.advance();
        }

        let end = self.idx;
        &self.src[start..end]
    }

    fn get_num(&mut self) -> &str {
        let start = self.idx;
        while self.current().is_digit(10) || self.current() == '.' {
            self.advance();
        }

        let end = self.idx;
        &self.src[start..end]
    }

    fn get_til_dblcolon(&mut self) -> &str {
        let start = self.idx;
        while self.current() != '"' {
            if self.current() == '\0' {
                panic!("encountered eof while parsing string literal");
            }
            self.advance();
        }

        let end = self.idx;
        &self.src[start..end]
    }

    fn advance_til_newline(&mut self) {
        while self.current() != '\n' {
            self.advance();
        }

        self.advance();
    }

    fn strip(&mut self) {
        while self.current() == ' '
            || self.current() == '\n'
            || self.current() == '\t'
            || self.current() == '\r'
        {
            self.advance();
        }
    }

    pub fn next(&mut self) -> Token {
        self.strip();
        if self.current() == self.comment_char {
            self.advance_til_newline();
        }
        self.strip();
        match self.current() {
            'a'..='z' | 'A'..='Z' => {
                let res = self.get_str();
                match res {
                    "proc" => Token::Fn,
                    "let" => Token::Let,
                    "return" => Token::Return,
                    _ => Token::Str(res.to_owned()),
                }
            }
            '0'..='9' => Token::Num(self.get_num().to_owned()),
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '{' => {
                self.advance();
                Token::LBrace
            }
            '}' => {
                self.advance();
                Token::RBrace
            }
            '[' => {
                self.advance();
                Token::LSqBracket
            }
            ']' => {
                self.advance();
                Token::RSqBracket
            }
            ';' => {
                self.advance();
                Token::SemiC
            }
            ':' => {
                self.advance();
                if self.current() == ':' {
                    self.advance();
                    Token::DoubleColon
                } else {
                    Token::Colon
                }
            }
            '!' => {
                self.advance();
                Token::ExclMark
            }
            '?' => {
                self.advance();
                Token::QuestMark
            }
            '=' => {
                self.advance();
                Token::Eq
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '*' => {
                self.advance();
                Token::Star
            }
            '/' => {
                self.advance();
                Token::Slash
            }
            '\\' => {
                self.advance();
                Token::BSlash
            }
            '+' => {
                self.advance();
                Token::Plus
            }
            '-' => {
                self.advance();
                Token::Minus
            }
            '"' => {
                self.advance();
                let text = self.get_til_dblcolon().to_owned();
                self.advance();
                Token::StrLit(text)
            }
            '\'' => {
                self.advance();
                Token::Quote
            }
            '&' => {
                self.advance();
                Token::Ampersand
            }
            '@' => {
                self.advance();
                Token::CommAt
            }
            '$' => {
                self.advance();
                Token::Dollar
            }

            '\0' => Token::EOF,
            _ => Token::Undef,
        }
    }
}
