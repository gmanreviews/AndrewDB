//mod lexer_impl;
//use lexer::


mod lexer;

fn main() {

    let test = "SELECT COL FROM MAIN";
    //let lexer = lexer::new();
    _ = lexer::Lexer::parse_text(test);

    println!("Hello, world!");
}
