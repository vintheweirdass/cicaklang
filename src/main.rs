use cicaklang::{lex::tokenize, util::PeekableWithPoint};
fn main() {
    match tokenize(&mut PeekableWithPoint::new("\"oo\"o023023 200.000")) {
        Ok(tokens) => { println!("{:#?}", tokens) }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
