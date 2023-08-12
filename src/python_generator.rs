use crate::parse::Node;
use crate::parse::NodeKind;
use crate::token::Type;
use std::collections::HashMap;
use std::process::exit;

use std::fs::File;
use std::io::{self, Read, Write};

const CONST_VARIABLE_RESERV: i32 = 0;
const CONST_FUNCTION_RESERV: i32 = 1;

fn str_to_string(s: &str) -> String {
    s.to_string()
}

pub struct PythonGenerator {
    tabs_counter: i32,
    source_buf: String,
    op_preset: HashMap<Type, String>,
    get_variable_or_function: HashMap<String, i32>,
    is_sucsess_type_test: bool,
    now_identifier: String,
    filename: String,
}

impl PythonGenerator {
    pub fn new(filename: String) -> Self {
        Self {
            tabs_counter: 0,
            source_buf: str_to_string(""),
            op_preset: [
                (Type::Greater, ">"),
                (Type::Less, "<"),
                (Type::Plus, "+"),
                (Type::Minus, "-"),
                (Type::Asterisk, "*"),
                (Type::Slash, "/"),
            ]
            .into_iter()
            .map(|(k, v)| (k, v.to_string()))
            .collect(),
            get_variable_or_function: HashMap::new(),
            is_sucsess_type_test: true,
            now_identifier: "".to_string(),
            filename,
        }
    }

    fn get_original_filename(&mut self, filename: String) -> String {
        if let Some(index) = filename.find('.') {
            filename[..index].to_string()
        } else {
            filename.to_string()
        }
    }

    pub fn write_to_file(&mut self, filename: String, content: &str) -> io::Result<()> {
        let mut file = File::create(filename)?; // ファイルを作成または開く

        file.write_all(content.as_bytes())?; // テキストをファイルに書き込む

        Ok(()) // 成功時は`Ok`を返す
    }

    pub fn add_source_buf(&mut self, data: String) {
        self.source_buf = format!("{}{}", self.source_buf, data);
    }

    pub fn exec_argument(&mut self, params: Vec<Node>) {
        for p in &params {
            self.generator(p.clone());
            self.get_variable_or_function
                .insert(self.now_identifier.clone(), CONST_VARIABLE_RESERV);

            if Some(p) != params.last() {
                self.add_source_buf(", ".to_string());
            } else {
                //                self.generator(p.clone());
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
                    self.add_source_buf(num.to_string());
                }
                NodeKind::Str(word) => {
                    self.now_identifier = word.clone();
                    self.add_source_buf(word);
                }
                NodeKind::Call {
                    function_name,
                    args,
                } => {
                    self.add_source_buf(function_name.clone());
                    match self.get_variable_or_function.get(&function_name) {
                        Some(value) => {
                            if *value == CONST_FUNCTION_RESERV {
                                // 関数が使われている
                                self.add_source_buf("(".to_string());
                                self.exec_argument(args);
                                self.add_source_buf(")".to_string());
                            } else {
                                // 変数が使われている
                            }
                        }
                        None => {
                            // 関数か変数かわからないものが使われている
                            println!("Err: {}が定義されていません", function_name);
                            self.is_sucsess_type_test = false;
                        }
                    }
                }
                NodeKind::Pass(word) => self.add_source_buf("pass".to_string()),
                NodeKind::Import(lib) => {
                    self.add_source_buf("import ".to_string());
                    self.add_source_buf(lib);
                }
                NodeKind::BinaryOp { op, lhs, rhs } => {
                    self.generator(*lhs);
                    self.add_source_buf(self.op_preset[&op].to_string());
                    self.generator(*rhs);
                }
                NodeKind::Return(arg) => {
                    self.add_source_buf("return ".to_string());
                    self.generator(*arg);
                }
                NodeKind::Compare { lhs, op, rhs } => {
                    self.generator(*lhs);
                    self.add_source_buf(self.op_preset[&*op].to_string());
                    self.generator(*rhs);
                }
                NodeKind::Let {
                    v_name,
                    v_type,
                    v_formula,
                    this_is_define,
                } => {
                    self.now_identifier = v_name.clone();
                    self.add_source_buf(v_name);
                    self.add_source_buf(":".to_string());
                    self.add_source_buf(v_type);
                    if this_is_define {
                        self.get_variable_or_function
                            .insert(self.now_identifier.clone(), CONST_VARIABLE_RESERV);
                        self.add_source_buf(" = ".to_string());
                    }
                    self.generator(*v_formula);
                }
                NodeKind::If { cond, then, else_ } => {
                    self.add_source_buf("if ".to_string());
                    self.generator(*cond);
                    self.add_source_buf(":\n".to_string());
                    self.generator(*then);
                }
                NodeKind::Expr { reserv } => {
                    let indent = self.get_indent().clone();
                    self.add_source_buf(indent.to_string());
                    self.generator(*reserv);
                    self.add_source_buf("\n".to_string());
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
                    let f_type = self.get_identifier(function_type);

                    self.add_source_buf("def ".to_string());
                    self.add_source_buf(identifier.to_string());
                    self.add_source_buf("(".to_string());
                    self.exec_argument(params);
                    self.add_source_buf(") ->".to_string());
                    self.add_source_buf(f_type.to_string());
                    self.add_source_buf(":\n".to_string());
                    self.generator(*body);
                }
                NodeKind::Root { function_define_s } => {
                    for ast in function_define_s {
                        self.generator(ast);
                    }
                    if !self.is_sucsess_type_test {
                        exit(0);
                    }
                    self.add_source_buf("main()".to_string());
                    let mut filename = self.filename.clone();
                    filename = self.get_original_filename(filename) + ".py";
                    let buf: &str = &self.source_buf.clone();
                    if let Err(e) = self.write_to_file(filename.clone(), buf) {
                        eprintln!("Error: {}", e);
                    } else {
                        println!("File '{}' created and written successfully.", filename);
                    }
                }
                _ => {}
            },
            None => {}
        }
    }
}
