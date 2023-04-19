use maplit::hashmap;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    SemiCoron,
    Coron,
    Equal,
    Less,
    Greater,
    Identifier(String),
    Number(f64),
}

pub struct Token {
    token_content: String,
    token_type: Type,
}

pub struct Lexer {
    tokens: Vec<Token>,
    code: String,
    char_to_type: HashMap<char, Type>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        let tokens = Vec::new();
        let types_hash = hashmap!(
            '+' => Type::Plus,
            '-' => Type::Minus,
            '*' => Type::Asterisk,
            '/' => Type::Slash,
            ';' => Type::SemiCoron,
            ':' => Type::Coron,
            '=' => Type::Equal,
            '(' => Type::LParen,
            ')' => Type::RParen,
            '<' => Type::Less,
            '>' => Type::Greater,
        );
        Self {
            tokens,
            code,
            char_to_type: types_hash,
        }
    }

    pub fn lex(&mut self) -> Vec<Type> {
        let mut tokens = vec![];
        let mut chars = &mut self.code.clone();
        let mut chars = chars.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if let Some(value) = self.char_to_type.get(&ch).cloned() {
                // `ch`が`char_to_type`のキーに存在する場合、`value`は`char_to_type[ch]`の値
                tokens.push(value);
                chars.next();
            } else {
                match ch {
                    '0'..='9' => tokens.push(self.parse_number(&mut chars).unwrap()),
                    ' ' | 'a'..='z' | 'A'..='Z' | '_' => {
                        if ch == ' ' {
                            chars.next();
                        } else {
                            tokens.push(self.parse_identifier(&mut chars).unwrap())
                        }
                    }
                    _ => panic!("Invalid character: {}", ch),
                }
            }
        }

        tokens
    }

    fn parse_identifier(&mut self, chars: &mut Peekable<Chars>) -> Option<Type> {
        let mut identifier = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                identifier.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        if identifier.is_empty() {
            None
        } else {
            Some(Type::Identifier(identifier))
        }
    }

    fn parse_number(&mut self, chars: &mut Peekable<Chars>) -> Option<Type> {
        let mut number = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                number.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        if number.is_empty() {
            None
        } else {
            Some(Type::Number(number.parse().unwrap()))
        }
    }
}
