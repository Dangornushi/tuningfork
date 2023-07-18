use std::process::exit;

use crate::token::Type;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
#[derive(Clone)]
pub enum NodeKind {
    Num(i32),
    Str(String),
    Pass(String),
    BinaryOp {
        op: Type,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    VarRef(String),
    Assign {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Compare {
        lhs: Box<Node>,
        op: Box<Type>,
        rhs: Box<Node>,
    },
    Block(Vec<Node>),
    Let {
        v_name: String,
        v_type: String,
        v_formula: Box<Node>,
    },
    If {
        cond: Box<Node>,
        then: Box<Node>,
        else_: Option<Box<Node>>,
    },
    While {
        cond: Box<Node>,
        body: Box<Node>,
    },
    Function {
        params: Vec<Node>,
        body: Box<Node>,
        function_type: Type,
        function_name: Type,
    },
    Call {
        function_name: Box<Node>,
        args: Vec<Node>,
    },
    Return(Box<Node>),
    Root {
        function_define_s: Vec<Node>,
    },
}
#[derive(Clone)]
pub struct Node {
    pub kind: Option<NodeKind>,
    pub token: Type,
}

impl Node {
    pub fn new() -> Self {
        Self {
            kind: None,
            token: Type::EOF,
        }
    }
}

pub struct Parser<'a> {
    pub node: Node,
    pub now_token: std::slice::Iter<'a, Type>,
    pub tokens: &'a [Type],
    pub function_argments: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Type]) -> Self {
        Self {
            node: Node::new(),
            now_token: tokens.iter(),
            tokens,
            function_argments: Vec::new(),
        }
    }

    fn expect(&mut self, expect_token: Type) -> bool {
        if self.now_token.next().unwrap().clone() == expect_token {
            true
        } else {
            false
        }
    }

    fn expect_err(&mut self, expect_token: Type) -> bool {
        if self.expect(expect_token.clone()) {
            true
        } else {
            panic!("Syntax error: {:?} がありません", expect_token)
        }
    }

    fn word(&mut self) -> Node {
        let mut token = self.now_token.clone();
        let mut token2 = self.now_token.clone();

        if let Type::Identifier(word) = token.next().unwrap().clone() {
            return Node {
                kind: Some(NodeKind::Str(word)),
                token: token.next().unwrap().clone(),
            };
        } else {
            panic!(
                "Typeが異なります!!!\n    予期されたタイプ: Identifier\n    確認されたタイプ: {:?}",
                token2.next().unwrap().clone()
            );
        }
    }
    fn number(&mut self) -> Node {
        let mut token = self.now_token.clone();

        println!("{:?}", self.now_token.clone().next());

        if let Type::Number(number) = token.next().unwrap().clone() {
            return Node {
                kind: Some(NodeKind::Num(number as i32)),
                token: self.now_token.next().unwrap().clone(),
            };
        } else {
            panic!(
                "予想外のトークン: {:?}",
                self.now_token.clone().next().unwrap().clone()
            );
        }
    }

    fn call_function(&mut self) -> Node {
        let function_name_node = self.word();
        self.now_token.next();
        let mut args = vec![Node {
            kind: None,
            token: Type::EOF,
        }];

        if *self.now_token.clone().next().unwrap() == Type::LParen {
            self.now_token.next();
            let next_token = self.now_token.clone().next().unwrap();
            if *next_token != Type::RParen {
                args = self.argument();
            }
            self.now_token.next();
        } else {
        }

        return Node {
            kind: Some(NodeKind::Call {
                function_name: Box::new(function_name_node),
                args,
            }),
            token: Type::EOF,
        };
    }

    fn binary_op(&mut self) -> Node {
        let lhs = self.call_function();
        let op;
        let rhs;

        if self.now_token.clone().next().unwrap().clone() == Type::Asterisk
            || self.now_token.clone().next().unwrap().clone() == Type::Slash
        {
            op = self.now_token.next().unwrap().clone();

            rhs = Box::new(self.binary_op());

            return Node {
                kind: Some(NodeKind::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs,
                }),
                token: Type::EOF,
            };
        }
        if self.now_token.clone().next().unwrap().clone() == Type::Plus
            || self.now_token.clone().next().unwrap().clone() == Type::Minus
        {
            op = self.now_token.next().unwrap().clone();

            rhs = Box::new(self.binary_op());

            return Node {
                kind: Some(NodeKind::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs,
                }),
                token: Type::EOF,
            };
        }
        if type_of(self.now_token.clone().next().unwrap()) == "Type::Identifier" {
            println!("OK");
            return lhs;
        } else {
            return lhs;
        }
    }

    fn err_message(&mut self, msg: String) {
        println!("Err:{}", msg);
    }

    fn boolean(&mut self) -> Node {
        let lhs = self.reserv();
        let op;

        match self.now_token.clone().next().unwrap() {
            Type::Greater | Type::Less => {
                op = self.now_token.next();
            }
            _ => {
                return lhs;
            }
        }
        let rhs = self.boolean();

        return Node {
            kind: Some(NodeKind::Compare {
                lhs: Box::new(lhs),
                op: Box::new(Type::Less),
                rhs: Box::new(rhs),
            }),
            token: Type::EOF,
        };
    }

    fn reserv(&mut self) -> Node {
        let reserv_token = self.now_token.clone().next().unwrap();
        let mut next_token_base = self.now_token.clone();
        next_token_base.next();
        let next_token = next_token_base.next().unwrap();

        if let Type::Colon = *next_token {
            // int: hoge
            let mut v_type = String::from("");
            let mut v_name = String::from("");
            let mut v_formula = Box::new(Node {
                kind: None,
                token: Type::EOF,
            });

            if let Type::Identifier(word) = reserv_token {
                v_type = word.to_string();
            }
            self.now_token.next();
            self.expect_err(Type::Colon);
            if let Type::Identifier(word) = self.now_token.next().unwrap() {
                v_name = word.to_string();
            }

            // int: hoge = hoge
            // TODO

            if *self.now_token.clone().next().unwrap() == Type::Equal {
                self.now_token.next();
                v_formula = Box::new(self.reserv());
            }

            return Node {
                kind: Some(NodeKind::Let {
                    v_name,
                    v_type,
                    v_formula,
                }),
                token: Type::EOF,
            };
        } else if let Type::Identifier(identifier) = reserv_token.clone() {
            match identifier.as_str() {
                "return" => {
                    self.now_token.next();
                    let arg_node = self.reserv();

                    let mut now_token = self.now_token.clone();

                    return Node {
                        kind: Some(NodeKind::Return(Box::new(arg_node))),
                        token: now_token.next().unwrap().clone(),
                    };
                }
                "if" => {
                    self.now_token.next();
                    let boolean = self.boolean();
                    let then = self.body();

                    return Node {
                        kind: Some(NodeKind::If {
                            cond: Box::new(boolean),
                            then: Box::new(then),
                            else_: None,
                        }),
                        token: Type::EOF,
                    };
                }
                "pass" => {
                    let word = String::from("Pass");
                    self.now_token.next();
                    return Node {
                        kind: Some(NodeKind::Pass(word)),
                        token: Type::EOF,
                    };
                }
                /*
                "while" => {
                    self.now_token.next();
                    self.expect(Type::LParen);
                    self.expect(Type::RParen);
                }
                 */
                _ => self.binary_op(),
            }
        } else {
            return self.number();
            /*
             */
        }
    }

    fn expr(&mut self) -> Node {
        let reserv = self.reserv();

        if self.expect_err(Type::SemiColon) {}

        reserv
    }

    fn body(&mut self) -> Node {
        let mut vec_node = Vec::new();

        self.expect_err(Type::LBraces);
        self.now_token.next();

        loop {
            let mut token = self.now_token.clone();

            if token.next().unwrap().clone() == Type::RBraces {
                break;
            }

            vec_node.push(self.expr());
            self.now_token.next();
        }

        Node {
            kind: Some(NodeKind::Block(vec_node)),
            token: self.now_token.next().unwrap().clone(),
        }
    }

    pub fn argument(&mut self) -> Vec<Node> {
        let mut arguments: Vec<Node> = Vec::new();
        loop {
            arguments.push(self.reserv());
            let mut token = self.now_token.clone();

            if token.next().unwrap().clone() == Type::Conma {
                self.now_token.next();
            } else {
                //                let mut token = self.now_token.clone();
                //                println!("OK: {:?}", token.next().unwrap().clone());
                //                self.now_token.next();
                break;
            }
        }
        arguments
    }

    pub fn function(&mut self) -> Node {
        let function_type = self.now_token.next().unwrap().clone();
        self.expect_err(Type::Colon);
        let function_name = self.now_token.next().unwrap().clone();
        self.expect_err(Type::LParen);
        let argument = self.argument();
        self.expect_err(Type::RParen);
        self.expect_err(Type::Equal);
        //        self.expect_err(Type::Minus);
        Node {
            kind: Some(NodeKind::Function {
                params: argument,
                body: Box::new(self.body()),
                function_type,
                function_name,
            }),
            token: Type::EOF,
        }
    }

    pub fn root(&mut self) -> Node {
        let mut function_define_s = Vec::new();
        self.now_token = self.tokens.iter();
        self.now_token.next();
        function_define_s.push(self.function());
        self.now_token.next();
        function_define_s.push(self.function());
        Node {
            kind: Some(NodeKind::Root { function_define_s }),
            token: Type::EOF,
        }
    }
}
