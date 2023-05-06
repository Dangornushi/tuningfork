mod parse;
mod token;

use crate::parse::Node;
use crate::parse::NodeKind;

fn gen(node: Node) {
    match node.kind.unwrap() {
        NodeKind::Num(num) => {
            println!("num: {:?}", num);
        }
        NodeKind::Str(word) => {
            println!("word: {:?}", word);
        }
        NodeKind::Return(arg) => {
            println!("Retrun: {{");
            gen(*arg);
            println!("Retrun: }}");
        }
        NodeKind::Block(block) => {
            println!("Block: {{");
            for b in block {
                gen(b);
            }
            println!("Block: }}");
        }
        _ => {}
    }
}

fn main() {
    let code_string = String::from(
        "
        {
a;
return x;
return y;
        }",
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

    println!(
        "===--- Compile ---==={}\n===--- END ---===\n AST:\n",
        code_string.clone()
    );

    let mut lexer = token::Lexer::new(code_string);
    let tokens = lexer.lex();
    let mut parse = parse::Parser::new(&tokens);
    let ast = parse.root();

    gen(ast);
}
