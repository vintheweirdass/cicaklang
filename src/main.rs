use cicaklang::{lex::tokenize, util::PeekableWithPoint};
fn main() {
    match tokenize(&mut PeekableWithPoint::new("\"\"")) {
        Ok(tokens) => { println!("{:#?}", tokens) }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
