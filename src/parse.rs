use crate::token::Type;

pub enum Operator {
    Plus,
    Minus,
    Slas,
    Asterisk,
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
}

pub enum NodeKind {
    Num(i32),
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
}

pub struct Node {
    pub kind: Option<NodeKind>,
    pub tokens: Vec<Type>,
}

impl Node {
    pub fn new(tokens: Vec<Type>) -> Self {
        Self {
            kind: None,
            tokens: tokens.clone(),
        }
    }

    // pub fn expr(&mut self) -> Node {}
    fn number(&mut self) -> Node {
        Node {
            kind: Some(NodeKind::Num(12)),
            tokens: self.tokens.clone(),
        }
    }

    fn body(&mut self) -> Node {
        let vec_node = vec![self.number(), self.number()];

        Node {
            kind: Some(NodeKind::Block(vec_node)),
            tokens: self.tokens.clone(),
        }
    }

    pub fn function(&mut self) -> Node {
        Node {
            kind: Some(NodeKind::Function {
                params: vec!["arg1".to_string(), "arg2".to_string()],
                body: Box::new(self.body()),
            }),
            tokens: self.tokens.clone(),
        }
    }
    pub fn root(&mut self) -> Node {
        let num_enum = Node {
            kind: Some(NodeKind::Num(12)),
            tokens: self.tokens.clone(),
        };

        let num_enum_2 = Node {
            kind: Some(NodeKind::Num(42)),
            tokens: self.tokens.clone(),
        };

        let vec_node = vec![num_enum, num_enum_2];

        let block_enum = Node {
            kind: Some(NodeKind::Block(vec_node)),
            tokens: self.tokens.clone(),
        };

        block_enum
    }
}
