use parser::Parser;

fn main() {
    let parser = Parser::new();
    parser.parse("Hello, world!".to_string());
}
