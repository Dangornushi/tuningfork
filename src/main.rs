mod parse;
mod token;

use crate::parse::Node;
use crate::parse::NodeKind;
use crate::token::Type;

fn get_identifier(type_data: Type) -> String {
    let nothing = String::from("");

    if let Type::Identifier(word) = type_data.clone() {
        return word;
    } else {
        return nothing;
    }
}

fn gen(node: Node) {
    match node.kind {
        Some(node_kind) => match node_kind {
            NodeKind::Num(num) => {
                println!("num: {:?}", num);
            }
            NodeKind::Str(word) => {
                println!("word: {:?}", word);
            }
            NodeKind::Pass(word) => {
                println!("!Pass!")
            }
            NodeKind::BinaryOp { op, lhs, rhs } => {
                println!("BinaryOp: {{");
                gen(*lhs);
                println!("op: {:?}", op);
                gen(*rhs);
                println!("BinaryOp: }}");
            }
            NodeKind::Return(arg) => {
                println!("Retrun: {{");
                gen(*arg);
                println!("Retrun: }}");
            }
            NodeKind::Compare { lhs, op, rhs } => {
                println!("Compare [");
                gen(*lhs);
                println!("{:?}", op);
                gen(*rhs);
                println!("]")
            }
            NodeKind::Let {
                v_name,
                v_type,
                v_formula,
            } => {
                println!("Let {}:{} [", v_name, v_type);
                gen(*v_formula);
                println!("]");
            }
            NodeKind::If { cond, then, else_ } => {
                println!("If [");
                gen(*cond);
                println!("]: {{");
                gen(*then);
                println!("If: }}");
            }
            NodeKind::Block(block) => {
                println!("Block: {{");
                for b in block {
                    gen(b);
                }
                println!("Block: }}");
            }
            NodeKind::Function {
                params,
                body,
                function_type,
                function_name,
            } => {
                print!("Function ");
                print!("{} ", get_identifier(function_name));
                print!("-> {}", get_identifier(function_type));
                print!(" [");
                for p in params {
                    gen(p);
                }
                println!("] {{");
                gen(*body);
                println!("Function: }}");
            }
            NodeKind::Root { function_define_s } => {
                for ast in function_define_s {
                    gen(ast);
                }
            }
            _ => {}
        },
        None => {}
    }
}

fn main() {
    let code_string = String::from(
        "
int: main(int: a, return b) <- {
    a;
    int: x <- 12;
    return x + y + z;
}
int: sub1(return c, return d) <- {
    x;
    if pass>pass<12 {
        return x;
    };
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
