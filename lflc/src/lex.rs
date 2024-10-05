use std::str::Split;
use std::str::Chars;

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
    YELL,
    CREATE,
    IS,
    ACTION,
    BINARY(char),
    SEPARATOR,
    END, // Ends blocks. TODO: take this from the indentation rather than a keyword
    ERROR
}

pub struct Lexer<'a> {
    lines: Box<Split<'a, &'a str>>,
    pos: usize,
    indentation: i32
}
impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            lines: Box::new(text.split("\n")),
            pos: 0,
            indentation: 0
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
        
        let mut has_ended = false;
        loop {
            let line = match self.lines.next() {
                Some(thing) => thing,
                None => break
            };
            loop {
                let token = self.next_token(line);
                result.push(token.clone());
                if token == Token::SEPARATOR {
                    break;
                }
                if token == Token::ERROR {
                    has_ended = true;
                    break;
                }
            }
            if has_ended {
                break;
            }
            self.pos = 0;
        }

        result
    }
    fn next_token(&mut self, line: &str) -> Token {
        let chars = &mut line[self.pos..].chars().peekable();
        let src = line;

        let mut pos = self.pos;

        // Note indentation
        loop {
            {
                let c = chars.peek();
                if c.is_none() {
                    return Token::SEPARATOR;
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
            return Token::SEPARATOR;
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
                    "create" => Token::CREATE,
                    ident => Token::IDENT(ident.to_string())
                }
            },
            //'0'..='9' | ('.' if chars.peek() == '0'..='9')  => {
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
            ';' => Token::SEPARATOR,
            ':' => Token::COLON,
            '.' => Token::PERIOD,
            '!' => Token::YELL,
            c @ ('+' | '-' | '/' | '*') => Token::BINARY(c),
            _ => {
                eprintln!("Unexpected token");
                Token::ERROR // Stop collecting tokens after unexpected token
            }
        };
        self.pos = pos;
        tok
    }
}