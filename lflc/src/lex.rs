use std::str::Chars;
use std::iter::Peekable;
use std::ops::DerefMut;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Token {
    IDENT(String),
    NUMBER(f64),
    LPAREN,
    RPAREN,
    COMMA,
    COLON,
    PERIOD,
    IS,
    ACTION,
    END, // Ends blocks. TODO: take this from the indentation rather than a keyword
    PLUS,
    MINUS,
    DIVIDE,
    MULTIPLY,
    EOF
}

pub struct Lexer<'a> {
    text: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    pos: usize
}
impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text: text,
            chars: Box::new(text.chars().peekable()),
            pos: 0
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
        loop {
            let token = self.next_token();
            result.push(token.clone());
            if token == Token::EOF {
                break;
            }
        }
        result
    }
    fn next_token(&mut self) -> Token {
        let chars = self.chars.deref_mut();
        let src = self.text;

        let mut pos = self.pos;

        // Note indentation
        loop {
            {
                let c = chars.peek();
                if c.is_none() {
                    return Token::EOF;
                }
                if !c.unwrap().is_whitespace() {
                    break;
                }
            }
            chars.next();
            pos += 1;
        }

        let start = pos;
        let next = chars.next();
        if next.is_none() {
            return Token::EOF;
        }
        pos += 1;
        let tok = match next.unwrap() {
            'a'..='z' | 'A'..='Z' | '_' => {
                loop {
                    let c = match chars.peek() {
                        Some(c) => *c,
                        None => '\0'
                    };
                    if c != '_' && !c.is_alphanumeric() {
                        break;
                    }
                    chars.next();
                    pos += 1;
                }
                match &src[start..pos] {
                    "is" => Token::IS,
                    "action" => Token::ACTION,
                    "end" => Token::END,
                    ident => Token::IDENT(ident.to_string())
                }
            },
            '0'..='9' => {
                loop {
                    let c = match chars.peek() {
                        Some(c) => *c,
                        None => '\0'
                    };
                    if c != '.' && !c.is_ascii_hexdigit() {
                        break;
                    }
                    chars.next();
                    pos += 1;
                }
                Token::NUMBER(src[start..pos].parse().unwrap())
            },
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            ':' => Token::COLON,
            '.' => Token::PERIOD,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '/' => Token::DIVIDE,
            '*' => Token::MULTIPLY,
            _ => {
                eprintln!("Unexpected token");
                Token::EOF // Stop collecting tokens after unexpected token
            }
        };
        self.pos = pos;
        tok
    }
}