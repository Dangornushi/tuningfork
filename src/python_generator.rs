use crate::parse::Node;
use crate::parse::NodeKind;
use crate::token::Type;
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;

const CONST_VARIABLE_RESERV: i32 = 0;
const CONST_FUNCTION_RESERV: i32 = 1;

fn str_to_string(s: &str) -> String {
    s.to_string()
}

pub struct PythonGenerator {
    tabs_counter: i32,
    python_source_buf: String,
    op_preset: HashMap<Type, String>,
    get_variable_or_function: HashMap<String, i32>,
    is_sucsess_type_test: bool,
}

impl PythonGenerator {
    pub fn new() -> Self {
        Self {
            tabs_counter: 0,
            python_source_buf: str_to_string(""),
            op_preset: [
                (Type::Greater, ">"),
                (Type::Less, "<"),
                (Type::Plus, "+"),
                (Type::Minus, "-"),
            ]
            .into_iter()
            .map(|(k, v)| (k, v.to_string()))
            .collect(),
            get_variable_or_function: HashMap::new(),
            is_sucsess_type_test: true,
        }
    }

    pub fn add_python_source_buf(&mut self, data: String) {
        self.python_source_buf = format!("{}{}", self.python_source_buf, data);
    }

    pub fn exec_argument(&mut self, params: Vec<Node>) {
        for p in &params {
            self.generator(p.clone());
            if Some(p) != params.last() {
                self.add_python_source_buf(", ".to_string());
            }
        }
    }

    pub fn get_identifier(&mut self, type_data: Type) -> String {
        let nothing = String::from("");

        if let Type::Identifier(word) = type_data.clone() {
            return word;
        } else {
            return nothing;
        }
    }

    pub fn get_indent(&mut self) -> String {
        let a_indent = "    ";
        let mut indent = str_to_string("");

        for _ in 0..self.tabs_counter {
            indent += a_indent;
        }

        indent
    }

    pub fn generator(&mut self, node: Node) {
        match node.kind {
            Some(node_kind) => match node_kind {
                NodeKind::Num(num) => {
                    self.add_python_source_buf(num.to_string());
                }
                NodeKind::Str(word) => {
                    self.add_python_source_buf(word);
                }
                NodeKind::Call {
                    function_name,
                    args,
                } => {
                    self.add_python_source_buf(function_name.clone());
                    match self.get_variable_or_function.get(&function_name) {
                        Some(value) => {
                            if *value == CONST_FUNCTION_RESERV {
                                self.add_python_source_buf("(".to_string());
                                self.exec_argument(args);
                                self.add_python_source_buf(")".to_string());
                            } else {
                            }
                        }
                        None => {
                            println!("Err: {}が定義されていません", function_name);
                            self.is_sucsess_type_test = false;
                        }
                    }
                }
                NodeKind::Pass(word) => self.add_python_source_buf("pass".to_string()),
                NodeKind::BinaryOp { op, lhs, rhs } => {
                    self.generator(*lhs);
                    self.add_python_source_buf(self.op_preset[&op].to_string());
                    self.generator(*rhs);
                }
                NodeKind::Return(arg) => {
                    self.add_python_source_buf("retrun ".to_string());
                    self.generator(*arg);
                }
                NodeKind::Compare { lhs, op, rhs } => {
                    self.generator(*lhs);
                    self.add_python_source_buf(self.op_preset[&*op].to_string());
                    self.generator(*rhs);
                }
                NodeKind::Let {
                    v_name,
                    v_type,
                    v_formula,
                    this_is_define,
                } => {
                    self.add_python_source_buf(v_name.clone());
                    if this_is_define {
                        self.get_variable_or_function
                            .insert(v_name, CONST_VARIABLE_RESERV);
                        self.add_python_source_buf(" = ".to_string());
                    }
                    self.generator(*v_formula);
                }
                NodeKind::If { cond, then, else_ } => {
                    self.add_python_source_buf("if ".to_string());
                    self.generator(*cond);
                    self.add_python_source_buf(":\n".to_string());
                    self.generator(*then);
                }
                NodeKind::Expr { reserv } => {
                    let indent = self.get_indent().clone();
                    self.add_python_source_buf(indent.to_string());
                    self.generator(*reserv);
                    self.add_python_source_buf("\n".to_string());
                }
                NodeKind::Block(block) => {
                    self.tabs_counter += 1;
                    for b in block {
                        self.generator(b);
                    }
                    self.tabs_counter -= 1;
                }
                NodeKind::Function {
                    params,
                    body,
                    function_type,
                    function_name,
                } => {
                    let identifier = self.get_identifier(function_name);
                    self.get_variable_or_function
                        .insert(identifier.clone().to_string(), CONST_FUNCTION_RESERV);

                    self.add_python_source_buf("def ".to_string());
                    self.add_python_source_buf(identifier.to_string());
                    self.add_python_source_buf("(".to_string());
                    self.exec_argument(params);
                    self.add_python_source_buf("):\n".to_string());
                    self.generator(*body);
                }
                NodeKind::Root { function_define_s } => {
                    for ast in function_define_s {
                        self.generator(ast);
                    }
                    if !self.is_sucsess_type_test {
                        exit(0);
                    }
                    println!("{}", self.python_source_buf);
                }
                _ => {}
            },
            None => {}
        }
    }
}
