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
    Period,
    Identifier(String),
    DoubleQuotation(String),
    Number(i64),
    Hashtag,
    Atsign,
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
            '.' => Type::Period,
            '#' => Type::Hashtag,
            '\n' => Type::Enter,
            '@' => Type::Atsign,
            '!' => Type::EOF,
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
            } else if ch == '"' {
                chars.next();
                let mut identifier = String::new();

                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        break;
                    } else {
                        identifier.push(ch);
                        chars.next();
                    }
                }
                tokens.push(Type::DoubleQuotation(identifier));
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

        (0x3040..=0x309F).contains(&c)   ||  // ひらがな
        (0x30A0..=0x30FF).contains(&c)   ||  // カタカナ
        (0x3400..=0x4DBF).contains(&c)   ||  // CJK統合漢字拡張A
        (0x4E00..=0x9FFF).contains(&c)   ||  // 基本漢字 + CJK統合漢字
        (0x20000..=0x2A6DF).contains(&c) ||  // CJK統合漢字拡張B ~ E
        (0x2A700..=0x2B73F).contains(&c) ||  // CJK統合漢字拡張F
        (0x2B740..=0x2B81F).contains(&c) ||  // CJK統合漢字拡張G
        (0x2B820..=0x2CEAF).contains(&c) ||  // CJK統合漢字拡張H ~ J
        (0xF900..=0xFAFF).contains(&c)   ||  // CJK互換漢字
        (0x2F800..=0x2FA1F).contains(&c) || // CJK互換漢字補助
        (0x4E00..=0x9FFF).contains(&c) // 漢字
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
