use crate::parse::Node;
use crate::parse::NodeKind;
use crate::token::Type;
use std::collections::HashMap;
use std::process::exit;

use std::fs::File;
use std::io::{self, Write};

const CONST_VARIABLE_RESERV: i32 = 0;
const CONST_FUNCTION_RESERV: i32 = 1;
const CONST_CLASS_RESERV: i32 = 2;

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

        if let Type::Identifier(word) = type_data {
            word
        } else {
            nothing
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
        if let Some(node_kind) = node.kind {
            match node_kind {
                NodeKind::Num(num) => {
                    self.add_source_buf(num.to_string());
                }
                NodeKind::Str(word) => {
                    self.now_identifier = word.clone();
                    self.add_source_buf(word);
                }
                NodeKind::CallMenber {
                    now_menber_name,
                    next,
                } => {
                    self.add_source_buf(now_menber_name);
                    self.add_source_buf(".".to_string());
                    self.generator(*next);
                }
                NodeKind::Call {
                    function_name,
                    args,
                } => {
                    self.add_source_buf(function_name.clone());
                    match self.get_variable_or_function.get(&function_name) {
                        Some(value) => {
                            if *value == CONST_FUNCTION_RESERV || *value == CONST_CLASS_RESERV {
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
                            println!("警告: {}が定義されていません。直接記入モジュールに含まれていれば問題ありません。", function_name);

                            self.add_source_buf("(".to_string());
                            self.exec_argument(args);
                            self.add_source_buf(")".to_string());
                            //self.is_sucsess_type_test = false;
                        }
                    }
                }
                NodeKind::Pass(_word) => self.add_source_buf("pass".to_string()),
                NodeKind::Import(import_messod_name) => {
                    self.add_source_buf("import ".to_string());
                    self.add_source_buf(import_messod_name);
                    self.add_source_buf("\n".to_string());
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
                    /*
                                        self.add_source_buf(": ".to_string());
                                        self.add_source_buf(v_type);
                    */
                    if this_is_define {
                        self.get_variable_or_function
                            .insert(self.now_identifier.clone(), CONST_VARIABLE_RESERV);
                        self.add_source_buf(" = ".to_string());
                    }
                    self.generator(*v_formula);
                }
                NodeKind::If {
                    cond,
                    then,
                    elif_then,
                    else_then,
                } => {
                    self.add_source_buf("if ".to_string());
                    self.generator(*cond);
                    self.add_source_buf(":\n".to_string());
                    self.generator(*then);
                    if else_then != None {
                        let indent = self.get_indent();
                        self.add_source_buf(indent);
                        self.add_source_buf("else:\n".to_string());
                        self.generator(*else_then.unwrap());
                    }
                }
                NodeKind::While { cond, body } => {
                    self.add_source_buf("while ".to_string());
                    self.generator(*cond);
                    self.add_source_buf(":\n".to_string());
                    self.generator(*body);
                }
                NodeKind::Expr { reserv } => {
                    let indent = self.get_indent();
                    self.add_source_buf(indent);
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
                    is_menber,
                } => {
                    let identifier = self.get_identifier(function_name);
                    self.get_variable_or_function
                        .insert(identifier.to_string(), CONST_FUNCTION_RESERV);
                    let f_type = self.get_identifier(function_type);
                    let indent = self.get_indent();
                    self.add_source_buf(indent);
                    self.add_source_buf("def ".to_string());

                    if identifier == "_init_".to_string() {
                        self.add_source_buf("__init__".to_string());

                        self.add_source_buf("(".to_string());
                        if is_menber {
                            self.add_source_buf("self".to_string());
                            if !params.is_empty() {
                                self.add_source_buf(", ".to_string());
                            }
                        }

                        self.exec_argument(params);
                        self.add_source_buf(")".to_string());
                    } else {
                        self.add_source_buf(identifier);

                        self.add_source_buf("(".to_string());
                        if is_menber {
                            self.add_source_buf("self".to_string());
                            if !params.is_empty() {
                                self.add_source_buf(", ".to_string());
                            }
                        }

                        self.exec_argument(params);
                        self.add_source_buf(") -> ".to_string());
                        self.add_source_buf(f_type);
                    }
                    self.add_source_buf(":\n".to_string());
                    self.generator(*body);
                    self.add_source_buf("\n\n".to_string());
                }
                NodeKind::Class {
                    class_name,
                    menbers,
                } => {
                    self.get_variable_or_function
                        .insert(class_name.to_string(), CONST_CLASS_RESERV);
                    self.add_source_buf("class ".to_string());
                    self.add_source_buf(class_name);
                    self.add_source_buf(":\n".to_string());
                    self.tabs_counter += 1;
                    for func in menbers {
                        self.generator(func);
                    }
                    self.tabs_counter -= 1;
                }

                NodeKind::RawLanguage {
                    language_type,
                    raw_data,
                } => {
                    self.add_source_buf(raw_data);
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
            }
        }
    }
}
