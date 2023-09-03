use crate::token::Type;

const RAW_DATA_TYPE_PYTHON: i32 = 0;
const RAW_DATA_TYPE_C: i32 = 1;
const RAW_DATA_TYPE_CPP: i32 = 2;
const RAW_DATA_TYPE_RUST: i32 = 3;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
#[derive(Clone, PartialEq)]
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
    CallMenber {
        now_menber_name: String,
        next: Box<Node>,
    },
    Block(Vec<Node>),
    Import(String),
    Let {
        v_name: String,
        v_type: String,
        v_formula: Box<Node>,
        this_is_define: bool,
    },
    If {
        cond: Box<Node>,
        then: Box<Node>,
        elif_then: Option<Vec<Box<Node>>>,
        else_then: Option<Box<Node>>,
    },
    Elif {
        cond: Box<Node>,
        then: Box<Node>,
        else_then: Option<Box<Node>>,
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
        is_menber: bool,
    },
    Call {
        function_name: String,
        args: Vec<Node>,
    },
    Return(Box<Node>),
    Expr {
        reserv: Box<Node>,
    },
    RawLanguage {
        language_type: i32,
        raw_data: String,
    },
    Class {
        class_name: String,
        menbers: Vec<Node>,
    },
    Root {
        function_define_s: Vec<Node>,
    },
}
#[derive(Clone, PartialEq)]
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
    pub now_function_is_menber: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Type]) -> Self {
        Self {
            node: Node::new(),
            now_token: tokens.iter(),
            tokens,
            function_argments: Vec::new(),
            now_function_is_menber: false,
        }
    }

    fn skip(&mut self, expect_token: Type) -> bool {
        let mut token = self.now_token.clone();

        if token.next().unwrap() == &expect_token {
            self.now_token.next();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expect_token: Type) -> bool {
        //        self.skip(Type::Enter);
        let mut token = self.now_token.clone();
        let t2 = token.next().unwrap().clone();
        if t2 == Type::Enter {
            self.now_token.next();
            self.now_token.next();
        }
        match self.now_token.next().unwrap().clone() == expect_token {
            true => true,
            false => false,
        }
    }

    fn expect_err(&mut self, expect_token: Type) -> bool {
        if self.expect(expect_token.clone()) {
            true
        } else {
            panic!("Syntax error: {:?} が期待されていました。", expect_token,)
        }
    }

    fn get_identifier_contents(&mut self, data: Type) -> Result<String, &'static str> {
        if let Type::Identifier(word) = data {
            return Ok(word);
        } else {
            return Err("err");
        }
    }

    pub fn sheek_token(&mut self, index: i32) -> Type {
        let mut tmp_token = self.now_token.clone();

        for _ in 0..index - 1 {
            tmp_token.next();
        }
        tmp_token.next().unwrap().clone()
    }

    pub fn sheek_watch_token(&mut self, index: i32) -> Type {
        let tmp_token = self.sheek_token(index);

        println!("{:?}", tmp_token);
        tmp_token
    }

    pub fn safty_sheek_watch_token(
        &mut self,
        now_token: std::slice::Iter<Type>,
        index: i32,
    ) -> Type {
        let mut tmp_token = now_token.clone();

        for _ in 0..index - 1 {
            tmp_token.next();
        }
        println!("{:?}", tmp_token.clone().next());
        tmp_token.next().unwrap().clone()
    }

    pub fn safety_sheek_token(&mut self, now_token: std::slice::Iter<Type>, index: i32) -> Type {
        let mut tmp_token = now_token.clone();

        for _ in 0..index - 1 {
            tmp_token.next();
        }
        tmp_token.next().unwrap().clone()
    }

    fn word(&mut self) -> Node {
        let token = self.now_token.clone();
        let mut token2 = self.now_token.clone();

        if let Type::Identifier(word) = token.clone().next().unwrap().clone() {
            Node {
                kind: Some(NodeKind::Str(word)),
                token: token.clone().next().unwrap().clone(),
            }
        } else {
            panic!(
                "Typeが異なります!!!\n    予期されたタイプ: {:?}\n    確認されたタイプ: {:?}",
                token.clone().next().unwrap().clone(),
                token2.next().unwrap().clone()
            );
        }
    }
    fn number(&mut self) -> Node {
        let mut token = self.now_token.clone();

        if let Type::DoubleQuotation(word) = self.now_token.clone().next().unwrap().clone() {
            let s = format!("\"{}\"", word);
            return Node {
                kind: Some(NodeKind::Str(s)),
                token: self.now_token.next().unwrap().clone(),
            };
        }
        if let Type::Number(number) = token.next().unwrap().clone() {
            return Node {
                kind: Some(NodeKind::Num(number as i32)),
                token: self.now_token.next().unwrap().clone(),
            };
        }
        if let Type::Identifier(string) = self.now_token.clone().next().unwrap().clone() {
            return Node {
                kind: Some(NodeKind::Str(string)),
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
        let mut token = self.now_token.clone();
        let function_name = match token.next().unwrap().clone() {
            Type::Identifier(word) => word,
            _ => String::new(),
        };
        let is_f = token.next().unwrap().clone();
        if is_f != Type::LParen {
            return self.number();
        }
        self.now_token.next();
        let mut args = vec![Node {
            kind: None,
            token: Type::EOF,
        }];
        let kind;
        if *self.now_token.clone().next().unwrap() == Type::LParen {
            self.now_token.next();
            let next_token = self.now_token.clone().next().unwrap();
            if *next_token != Type::RParen {
                args = self.argument();
            }
            self.now_token.next();
            kind = Some(NodeKind::Call {
                function_name,
                args,
            });
        } else {
            kind = Some(NodeKind::Str(function_name));
        }

        Node {
            kind,
            token: Type::EOF,
        }
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

            //
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
            lhs
        } else {
            lhs
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

        Node {
            kind: Some(NodeKind::Compare {
                lhs: Box::new(lhs),
                op: Box::new(op.unwrap().clone()),
                rhs: Box::new(rhs),
            }),
            token: Type::EOF,
        }
    }

    fn reserv(&mut self) -> Node {
        let reserv_token = self.now_token.clone().next().unwrap();
        let mut next_token_base = self.now_token.clone();
        next_token_base.next();
        let next_token = next_token_base.next().unwrap();

        if let Type::Period = *next_token {
            let token = self.now_token.next().unwrap().clone();
            let this_menber_name = self.get_identifier_contents(token);
            self.now_token.next();
            Node {
                kind: Some(NodeKind::CallMenber {
                    now_menber_name: this_menber_name.unwrap(),
                    next: Box::new(self.reserv()),
                }),
                token: Type::EOF,
            }
        } else if let Type::Colon = *next_token {
            // int: hoge
            let mut v_type = String::from("");
            let mut v_name = String::from("");
            let mut v_formula = Box::new(Node {
                kind: None,
                token: Type::EOF,
            });
            let mut this_is_define = false;

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
                this_is_define = true;
            }

            Node {
                kind: Some(NodeKind::Let {
                    v_name,
                    v_type,
                    v_formula,
                    this_is_define,
                }),
                token: Type::EOF,
            }
        } else if let Type::Identifier(identifier) = reserv_token.clone() {
            match identifier.as_str() {
                "return" => {
                    self.now_token.next();
                    let arg_node = self.reserv();

                    let mut now_token = self.now_token.clone();

                    Node {
                        kind: Some(NodeKind::Return(Box::new(arg_node))),
                        token: now_token.next().unwrap().clone(),
                    }
                }
                "if" => {
                    let mut else_then: Option<Box<Node>> = None;
                    let mut elif_then: Option<Vec<Box<Node>>> = None;
                    self.now_token.next();
                    let boolean = self.boolean();
                    let then = self.body();
                    self.skip(Type::Enter);

                    if let Type::Identifier(identifier) = self.now_token.clone().next().unwrap() {
                        match identifier.as_str() {
                            "else" => {
                                self.now_token.next();
                                else_then = Some(Box::new(self.body()));
                            }
                            "elif" => {
                                self.now_token.next();
                                let boolean = self.boolean();
                                //elif_then.push(Some(Box::new(self.body())));
                            }
                            _ => else_then = None,
                        }
                    } else {
                        else_then = None;
                    }
                    Node {
                        kind: Some(NodeKind::If {
                            cond: Box::new(boolean),
                            then: Box::new(then),
                            elif_then,
                            else_then,
                        }),
                        token: Type::EOF,
                    }
                }
                "while" => {
                    self.now_token.next();
                    let boolean = self.boolean();
                    self.expect_err(Type::Equal);
                    let body = self.body();
                    Node {
                        kind: Some(NodeKind::While {
                            cond: Box::new(boolean),
                            body: Box::new(body),
                        }),
                        token: Type::EOF,
                    }
                }
                "pass" => {
                    let word = String::from("Pass");
                    self.now_token.next();
                    Node {
                        kind: Some(NodeKind::Pass(word)),
                        token: Type::EOF,
                    }
                }
                _ => self.binary_op(),
            }
        } else {
            self.number()
            /*
             */
        }
    }

    fn expr(&mut self) -> Node {
        if let Type::Enter = self.now_token.clone().next().unwrap().clone() {
            self.now_token.next();
            return self.expr();
        }
        let reserv = self.reserv();

        if self.expect_err(Type::SemiColon) {}

        Node {
            kind: Some(NodeKind::Expr {
                reserv: Box::new(reserv),
            }),
            token: Type::EOF,
        }
    }

    fn body(&mut self) -> Node {
        let mut vec_node = Vec::new();

        self.expect_err(Type::LBraces);
        self.now_token.next();

        loop {
            self.skip(Type::Enter);
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
            if let Some(next_token) = self.now_token.clone().next() {
                if self.get_identifier_contents(next_token.clone()) == Ok("self".to_string()) {
                    self.now_function_is_menber = true;
                    self.now_token.next();
                } else {
                    let reserv = self.reserv();
                    arguments.push(reserv);
                }
            }
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

        if let Type::Identifier(identifier) = function_type.clone() {
            if let "import" = identifier.as_str() {}
        }
        self.expect_err(Type::Colon);
        let function_name = self.now_token.next().unwrap().clone();

        let mut token = self.now_token.clone();
        let argument;
        if token.next().unwrap() == &Type::LParen {
            self.expect_err(Type::LParen);
            argument = self.argument();
            self.expect_err(Type::RParen);
        } else {
            argument = vec![Node {
                kind: None,
                token: Type::EOF,
            }];
        }
        let now_function_is_menber = self.now_function_is_menber.clone();
        self.now_function_is_menber = false;

        self.expect_err(Type::Equal);

        Node {
            kind: Some(NodeKind::Function {
                params: argument,
                body: Box::new(self.body()),
                function_type,
                function_name,
                is_menber: now_function_is_menber,
            }),
            token: Type::EOF,
        }
    }

    pub fn class(&mut self) -> Node {
        self.now_token.next();
        let tmp = self.now_token.next().unwrap().clone();
        let get_class_name = self.get_identifier_contents(tmp);
        let mut class_name: String = "".to_string();

        if let Ok(data) = get_class_name {
            class_name = data;
        } else if let Err(error) = get_class_name {
            println!("Error: {}", error);
        }

        self.expect_err(Type::Equal);
        self.expect_err(Type::LBraces);

        let mut menber_s = Vec::new();

        loop {
            match self.enter_skip() {
                Err("err") => {
                    break;
                }
                Ok("function") => {
                    menber_s.push(self.function());
                    if self.sheek_token(2) == Type::RBraces {
                        break;
                    }
                }
                _ => {}
            }
            let _ = self.enter_skip();
        }

        Node {
            kind: Some(NodeKind::Class {
                class_name,
                menbers: menber_s,
            }),
            token: Type::EOF,
        }
    }

    pub fn import(&mut self) -> Node {
        self.now_token.next();
        let mut import_messod = String::from("");
        if let Type::Identifier(import_messod_name) = self.now_token.next().unwrap() {
            import_messod = import_messod_name.to_string();
        }
        Node {
            kind: Some(NodeKind::Import(ToString::to_string(&import_messod))),
            token: Type::EOF,
        }
    }

    pub fn macro_raw_data(&mut self) -> Node {
        self.now_token.next();
        self.expect_err(Type::Colon);
        let language_type = self.now_token.next();
        self.expect_err(Type::Equal);
        self.expect_err(Type::LBraces);

        let mut raw_data: String = String::new();
        self.skip(Type::Enter);

        if let Type::DoubleQuotation(word) = self.now_token.clone().next().unwrap() {
            raw_data = word.to_string();
        }
        self.now_token.next();

        Node {
            kind: Some(NodeKind::RawLanguage {
                language_type: RAW_DATA_TYPE_PYTHON,
                raw_data,
            }),
            token: Type::EOF,
        }
    }

    pub fn enter_skip(&mut self) -> Result<&str, &str> {
        let t2 = self.now_token.clone().next();
        if t2.is_none() {
            // Fileの終わり
            return Err("err");
        }
        if let Type::RBraces = t2.unwrap() {
            self.now_token.next();
            return Ok("}");
        }

        if let Type::Identifier(type_or_import) = t2.unwrap() {
            match type_or_import.as_str() {
                "import" => Ok("import"),
                "language" => Ok("language"),
                "class" => Ok("class"),
                _ => Ok("function"),
            }
        } else {
            self.now_token.next();
            //let _ =
            return self.enter_skip();

            //    self.sheek_watch_token(2);
        }
    }

    pub fn root(&mut self) -> Node {
        let mut function_define_s = Vec::new();
        self.now_token = self.tokens.iter();

        loop {
            match self.enter_skip() {
                Err("err") => {
                    break;
                }
                Ok("function") => {
                    function_define_s.push(self.function());
                }
                Ok("class") => {
                    function_define_s.push(self.class());
                }
                Ok("import") => {
                    function_define_s.push(self.import());
                }
                Ok("language") => {
                    function_define_s.push(self.macro_raw_data());
                }
                _ => {}
            }
            let _ = self.enter_skip();
        }

        Node {
            kind: Some(NodeKind::Root { function_define_s }),
            token: Type::EOF,
        }
    }
}
