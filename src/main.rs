mod parse;
mod token;

use crate::parse::NodeKind;

fn main() {
    let code_string = String::from(
        "
return x;
return y;",
    );
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

    let mut lexer = token::Lexer::new(code_string);
    let tokens = lexer.lex();
    let mut parse = parse::Parser::new(&tokens);
    let ast = parse.root();

    match ast.kind.unwrap() {
        NodeKind::Num(num) => {
            println!("1: {:?}", num);
        }
        NodeKind::Block(block) => {
            println!("Block: {{");
            for b in block {
                println!("Retrun: {{");

                match b.kind.unwrap() {
                    NodeKind::Return(arg) => match arg.kind.unwrap() {
                        NodeKind::Num(num) => {
                            println!("num: {:?}", num);
                        }
                        NodeKind::Str(word) => {
                            println!("word: {:?}", word);
                        }

                        _ => {}
                    },
                    _ => {}
                }
                println!("Retrun: }}");
            }
            println!("Block: }}");
        }
        NodeKind::Function { params, body } => {
            for p in params {
                println!("arguments: {}", p);
            }

            println!("Function: {{");

            if let NodeKind::Block(block) = body.kind.unwrap() {
                println!("Block: {{");
                for b in block {
                    println!("Retrun: {{");

                    match b.kind.unwrap() {
                        NodeKind::Return(arg) => {
                            if let NodeKind::Num(num) = arg.kind.unwrap() {
                                println!("num: {:?}", num);
                            }
                        }
                        _ => {}
                    }
                    println!("Return: }}");
                }
                println!("Block: }}");
            }

            println!("Function: }}");
        }
        _ => {}
    }
}
