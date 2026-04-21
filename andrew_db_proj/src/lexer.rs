

//use crate::lexer::Identifier::{Select, Unknown};

enum Identifier
{
    Select,
    Unknown
}

pub struct Token
{
    identifier: Identifier,
    value: String
}

pub struct Lexer
{
    tokens: Vec<Token>,
}

impl Lexer
{
    /*
    pub fn new() -> Lexer
    {
        //to figure this out one day
        Lexer { tokens: vec![] }
    }
    */
     

    pub fn new(t: Vec<Token> ) -> Lexer
    {
        //to figure this out one day
        Lexer { tokens: t }
    }

    pub fn parse_text(text: &str) -> Lexer
    {
        //time to parse this stuff

        println!("Parsing text... {text}");

        let tokens = Self::parse_tokens(text);

        let lexer = Lexer::new(tokens);

        return lexer;
    }

    fn parse_tokens(text: &str) -> Vec<Token>
    {
        let mut tokens = vec![];
        let mut started = false;
        let mut start = 0;
        for (i, c) in text.char_indices() {
            if (c.is_whitespace() && !started)
            {
                continue;
            }

            if (!c.is_whitespace() && !started)
            {
                start = i;
                started = true;
            }

            if (c.is_whitespace() && started)
            {
                let t = &text[start..i];
                let token = Self::parse_token(t);
                //tokens.push(token);
                tokens.push(crate::lexer::Token {
                    identifier : token,
                    value: t.to_string()
                });
                started = false;
            }

        }
        let result = tokens;
        return result;
    }

    fn parse_token(text: &str) -> Identifier
    {
        if (text == "SELECT")
        {
            return Identifier::Select
        }

        Identifier::Unknown
    }


}