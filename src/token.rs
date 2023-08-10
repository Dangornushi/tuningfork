use maplit::hashmap;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Type {
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    SemiColon,
    Colon,
    Equal,
    Less,
    Greater,
    Enter,
    LBraces,
    RBraces,
    Conma,
    Identifier(String),
    Number(i64),
    EOF,
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
            ';' => Type::SemiColon,
            ':' => Type::Colon,
            '=' => Type::Equal,
            '(' => Type::LParen,
            ')' => Type::RParen,
            '<' => Type::Less,
            '>' => Type::Greater,
            '{' => Type::LBraces,
            '}' => Type::RBraces,
            ',' => Type::Conma,
            '\n' => Type::Enter,
        );
        Self {
            tokens,
            code,
            char_to_type: types_hash,
        }
    }

    pub fn lex(&mut self) -> Vec<Type> {
        let mut tokens = vec![];
        let chars = &mut self.code.clone();
        let mut chars = chars.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if ch == '<' {
                let mut chars2 = chars.clone();
                chars2.next();
                let ch2 = chars2.next();

                if ch2.unwrap() == '-' {
                    tokens.push(Type::Equal);
                    chars.next();
                } else {
                    tokens.push(Type::Less);
                }
                chars.next();
            } else if let Some(value) = self.char_to_type.get(&ch).cloned() {
                // `ch`が`char_to_type`のキーに存在する場合、`value`は`char_to_type[ch]`の値
                tokens.push(value);
                chars.next();
            } else {
                match ch {
                    // tokenに数字をプッシュ
                    '0'..='9' => tokens.push(self.parse_number(&mut chars).unwrap()),
                    ' '
                    | 'a'..='z'
                    | 'A'..='Z'
                    | '_'
                    | '\u{3040}'..='\u{309F}'
                    | '\u{4E00}'..='\u{9FFF}' => {
                        if ch == ' ' {
                            // tokenをスキップ
                            chars.next();
                        } else {
                            // 単語ごとに区切られた文字列をTokensにプッシュする
                            let words = self.parse_identifier(&mut chars).unwrap();
                            tokens.push(words);
                        }
                    }
                    '\t' => (),

                    _ => panic!("Invalid character: {}", ch),
                }
            }
        }

        tokens
    }

    pub fn is_japanese_char(&mut self, c: char) -> bool {
        let c = c as u32;

        (0x3040 <= c && c <= 0x309F)   ||  // ひらがな
        (0x30A0 <= c && c <= 0x30FF)   ||  // カタカナ
        (0x3400 <= c && c <= 0x4DBF)   ||  // CJK統合漢字拡張A
        (0x4E00 <= c && c <= 0x9FFF)   ||  // 基本漢字 + CJK統合漢字
        (0x20000 <= c && c <= 0x2A6DF) ||  // CJK統合漢字拡張B ~ E
        (0x2A700 <= c && c <= 0x2B73F) ||  // CJK統合漢字拡張F
        (0x2B740 <= c && c <= 0x2B81F) ||  // CJK統合漢字拡張G
        (0x2B820 <= c && c <= 0x2CEAF) ||  // CJK統合漢字拡張H ~ J
        (0xF900 <= c && c <= 0xFAFF)   ||  // CJK互換漢字
        (0x2F800 <= c && c <= 0x2FA1F) || // CJK互換漢字補助
        (0x4E00 <= c && c <= 0x9FFF) // 漢字
    }

    fn parse_identifier(&mut self, chars: &mut Peekable<Chars>) -> Option<Type> {
        let mut identifier = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_alphanumeric() || self.is_japanese_char(ch) || ch == '_' {
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
