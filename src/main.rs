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
                match b.kind.unwrap() {
                    NodeKind::Num(num) => {
                        println!("block: {:?}", num);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
