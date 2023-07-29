mod c_generator;
mod parse;
mod python_generator;
mod token;

fn run(code_string: String) {
    let mut lexer = token::Lexer::new(code_string);
    let tokens = lexer.lex();
    let mut parse = parse::Parser::new(&tokens);
    let ast = parse.root();

    //let mut generator = c_generator::C_Generator::new();
    let mut generator = python_generator::PythonGenerator::new();
    generator.generator(ast)
}

fn main() {
    let code_string = String::from(
        "
int: c(int: a, int: d) <- {
    int: x <- 12;
    pass;
}



int: main(int: a) <- {
    c(12);
    int: x <- 12;
    if a>x<12 {
        return x;
    };
    return c(12) + x;
}

",
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

    run(code_string);
}
