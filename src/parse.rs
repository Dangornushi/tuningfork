use crate::token::Type;

pub enum Operator {
    Plus,
    Minus,
    Slas,
    Asterisk,
    Colon,
    SemiColon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
}

pub enum NodeKind {
    Num(i32),
    Str(String),
    UnaryOp {
        op: Operator,
        operand: Box<Node>,
    },
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
    Block(Vec<Node>),
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
        params: Vec<String>,
        body: Box<Node>,
        function_type: Type,
        function_name: Type,
    },
    Call {
        function_type: Box<Node>,
        args: Vec<Node>,
    },
    Return(Box<Node>),
    Root {
        function_define_s: Vec<Node>,
    },
}

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
        Node {
            kind: Some(NodeKind::Num(12)),
            token: self.now_token.next().unwrap().clone(),
        }
    }

    fn call_function(&mut self) -> Node {
        let word_node = self.word();
        self.now_token.next();

        return word_node;
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
        } else {
            return lhs;
        }
    }

    fn reserv(&mut self) -> Node {
        let reserv_token = self.now_token.clone();

        if let Type::Identifier(identifier) = reserv_token.clone().next().unwrap().clone() {
            match identifier.as_str() {
                "if" => {
                    self.now_token.next();
                    let arg_node = self.reserv();

                    return Node {
                        kind: Some(NodeKind::Return(Box::new(arg_node))),
                        token: self.now_token.next().unwrap().clone(),
                    };
                }

                "return" => {
                    self.now_token.next();
                    let arg_node = self.reserv();

                    let mut now_token = self.now_token.clone();

                    return Node {
                        kind: Some(NodeKind::Return(Box::new(arg_node))),
                        token: now_token.next().unwrap().clone(),
                    };
                }

                _ => self.binary_op(),
            }
        } else {
            panic!(
                "予想外のトークン: {:?}",
                reserv_token.clone().next().unwrap().clone()
            );
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

    pub fn argment(&mut self) -> Vec<Node> {
        let mut arg;
        let mut argments: Vec<Node> = Vec::new();
        loop {
            argments.push(self.reserv());
            if self.expect(Type::Conma) {
            } else {
                break;
            }
        }
        argments
    }

    pub fn function(&mut self) -> Node {
        let function_type = self.now_token.next().unwrap().clone();
        self.expect_err(Type::Colon);
        let function_name = self.now_token.next().unwrap().clone();
        Node {
            kind: Some(NodeKind::Function {
                params: vec!["arg1".to_string(), "arg2".to_string()],
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
