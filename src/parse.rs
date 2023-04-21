use crate::token::Type;

pub struct Parser {
    tokens: Vec<Type>,
}

impl Parser {
    pub fn new(tokens: Vec<Type>) -> Self {
        Self { tokens }
    }
    pub fn parse(&mut self) {
        let mut index = 0;

        loop {
            if index >= self.tokens.len() {
                break;
            }
            let token = &self.tokens[index];
            match token {
                Type::Identifier(identifier) => {
                    match identifier.as_str() {
                        "int" => {
                            println!("Type: {:?}", token);
                            // do something
                        }
                        "float" => {
                            // do something else
                        }
                        _ => {
                            println!("var: {:?}", token);
                        }
                    }
                }
                Type::Number(_) => {
                    println!("num: {:?}", token);
                }
                Type::Less => {
                    // tokenを一つ進める
                    if let Some(next_item) = &self.tokens.get(index + 1) {
                        if next_item == &&Type::Minus {
                            println!("equal: <-");
                            index += 1;
                        } else {
                            println!("less: {:?}", token);
                        }
                    }
                }
                Type::SemiCoron => {
                    println!("semicoron: ;");
                }
                Type::Minus => {
                    println!("minus: -");
                }
                _ => {}
            }
            index += 1;
        }
    }
}
