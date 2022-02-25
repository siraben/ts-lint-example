use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_formula() -> Language;
}

fn main() {
    let mut args = std::env::args();
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_formula() };
    parser.set_language(language).unwrap();
    let file = args.nth(1).unwrap();
    let bytes = std::fs::read(&file).unwrap();
    println!("Parse tree for {}", file);
    print!(
        "{}",
        parser.parse(&bytes, None).unwrap().root_node().to_sexp()
    );
}
