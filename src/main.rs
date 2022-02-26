use tree_sitter::{Language, Parser, Query, QueryCursor, TextProvider};

extern "C" {
    fn tree_sitter_imp() -> Language;
}

fn main() {
    let mut args = std::env::args();
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_imp() };
    parser.set_language(language).unwrap();
    let file = args.nth(1).unwrap();
    let bytes = std::fs::read(&file).unwrap();
    let tree = parser.parse(&bytes, None).unwrap();
    // Match statements like x := x
    let query_str = "((asgn name: (id) @left _ @right) (#eq? @left @right)) @red";
    let query = Query::new(language, query_str).unwrap();
    println!("Redundant assignments:");
    let mut query_cursor = QueryCursor::new();
    let source = std::fs::read_to_string(file).unwrap();
    let matches = query_cursor.captures(&query, tree.root_node(), source.as_bytes());
    let mut redundant_assignments = matches
        .flat_map(|(m, _)| m.captures)
        .filter_map(|c| c.node.child_by_field_name("name"))
        .map(|c| {
            format!(
                "Redundant assignment to {} at row {} column {}",
                source[c.byte_range()].to_string(),
                c.start_position().row,
                c.start_position().column
            )
        })
        .collect::<Vec<_>>();
    redundant_assignments.dedup();
    println!("{}", redundant_assignments.join("\n"));
}
