mod token;

use token::Token;

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, source: String) -> Result<(), Vec<Token>> {
        println!("{}", source);
        Ok(())
    }
}
