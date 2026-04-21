pub struct Lexer
{
    //identifiers: Vec<Identifier>,
}

impl Lexer
{
    pub fn new() -> Lexer
    {
        //to figure this out one day
        Lexer { } //identifiers: vec![] }
    }

    pub fn parse_text(text: &str) -> Lexer
    {
        //time to parse this stuff

        println!("Parsing text... {text}");
        let lexer = Lexer::new();
        return lexer;
    }


}