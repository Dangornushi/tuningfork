mod parse;
mod token;

fn main() {
    let code_string = String::from("int: x <- 12;int: y <- 42;");

    let mut lexer = token::Lexer::new(code_string);
    let tokens = lexer.lex();
    let mut parser = parse::Parser::new(tokens.clone());
    let ast = parser.parse();
}
