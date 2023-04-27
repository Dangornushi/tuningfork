mod parse;
mod token;

use crate::parse::NodeKind;

fn main() {
    let code_string = String::from(
        "
return x
return y",
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
    let mut node = parse::Node::new(tokens.clone());
    let ast = node.root();

    match ast.kind.unwrap() {
        NodeKind::Num(num) => {
            println!("1: {:?}", num);
        }
        NodeKind::Block(block) => {
            for b in block {
                if let NodeKind::Num(num) = b.kind.unwrap() {
                    println!("block: {:?}", num);
                }
            }
        }
        NodeKind::Function { params, body } => {
            for p in params {
                println!("arguments: {}", p);
            }

            println!("Function: {{");

            if let NodeKind::Block(block) = body.kind.unwrap() {
                println!("Block: {{");
                for b in block {
                    if let NodeKind::Return { callee, args } = b.kind.unwrap() {
                        println!("Retrun: {{");

                        for c in args {
                            if let NodeKind::Num(num) = c.kind.unwrap() {
                                println!("num: {:?}", num);
                            }
                        }
                        println!("Return: }}");
                    }
                }
                println!("Block: }}");
            }

            println!("Function: }}");
        }
        _ => {}
    }
}
