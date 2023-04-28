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
        op: Operator,
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
    },
    Call {
        callee: Box<Node>,
        args: Vec<Node>,
    },
    Return(Box<Node>),
}

pub struct Node {
    pub kind: Option<NodeKind>,
    pub token: Type,
}

impl Node {
    pub fn new(tokens: Vec<Type>) -> Self {
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
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Type]) -> Self {
        Self {
            node: Node::new(tokens.to_vec()),
            now_token: tokens.iter(),
            tokens,
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
            panic!("Syntax error: {:?} is missing.", expect_token)
        }
    }

    fn word(&mut self) -> Node {
        let mut token = self.now_token.clone();

        if let Type::Identifier(word) = token.next().unwrap().clone() {
            return Node {
                kind: Some(NodeKind::Str(word)),
                token: token.next().unwrap().clone(),
            };
        } else {
            panic!("")
        }
    }
    fn number(&mut self) -> Node {
        Node {
            kind: Some(NodeKind::Num(12)),
            token: self.now_token.next().unwrap().clone(),
        }
    }

    fn reserv_words(&mut self, identifier: String) -> Node {
        match identifier.clone() {
            "return".to_string() => {
                let ret = self.reserv();
            }
            _ => {}
        }
        return ret;
    }

    fn reserv(&mut self) -> Node {
        match self.now_token.next().unwrap() {
            Type::Identifier(identifier) => {
                let arg_node = reserv_words(identifier);
            }
            Type::Number(n) => {
                let arg_node = self.number();
            }
            _ => {}
        }

        return Node {
            kind: Some(NodeKind::Return(Box::new(arg_node))),
            token: self.now_token.next().unwrap().clone(),
        };
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

    pub fn function(&mut self) -> Node {
        Node {
            kind: Some(NodeKind::Function {
                params: vec!["arg1".to_string(), "arg2".to_string()],
                body: Box::new(self.body()),
            }),
            token: self.now_token.next().unwrap().clone(),
        }
    }

    pub fn root(&mut self) -> Node {
        self.now_token = self.tokens.iter();
        self.now_token.next();
        self.body() //function()
    }
}
