mod c_generator;
mod parse;
mod python_generator;
mod token;

use std::fs::File;
use std::io::{self, Read, Write};

fn read_from_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?; // ファイルを開く

    let mut content = String::new();
    file.read_to_string(&mut content)?; // ファイルからテキストを読み込む

    Ok(content) // 読み込んだテキストを`Ok`で返す
}

fn run(code_string: String) {
    let mut lexer = token::Lexer::new(code_string);
    let tokens = lexer.lex(); // Token列を作成
    let mut parse = parse::Parser::new(&tokens);
    let ast = parse.root(); // AST列を作成

    //let mut generator = c_generator::C_Generator::new();
    let mut generator = python_generator::PythonGenerator::new(); // Python generator
                                                                  // のインスタンスを作成
    generator.generator(ast) // AST列を解析
}

fn main() {
    let filename = "test.txt";
    match read_from_file(filename) {
        Ok(code_string) => run(code_string),
        Err(e) => eprintln!("Error: {}", e),
    }
    /*"
    int: main() <- {
        int: x <- 12;
        int: y <- 42;

        // 結果は54になるはず
        return x + y;
    }
    ",
        );
        */
}
