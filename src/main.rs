use tree_sitter::{Language, Parser, Query, QueryCursor};

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
    let source = std::fs::read_to_string(file).unwrap();
    println!("Redundant assignments:");
    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), source.as_bytes());
    let redundant_assignments = matches
        .flat_map(|m| m.captures)
        .filter_map(|c| c.node.child_by_field_name("name"))
        .map(|c| {
            format!(
                "Redundant assignment to {} at line {} column {}",
                source[c.byte_range()].to_string(),
                c.start_position().row + 1,
                c.start_position().column + 1
            )
        })
        .collect::<Vec<_>>();
    println!("{}", redundant_assignments.join("\n"));
    // Redundant if
    let query_str = "((if condition: _ @c consequent: _ @l alternative: _ @r) (#eq? @l @r)) @if2";
    let query = Query::new(language, query_str).unwrap();
    println!("Redundant if statement:");
    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), source.as_bytes());
    let redundant_assignments = matches
        .flat_map(|m| m.captures)
        .filter_map(|c| {
            if c.node.kind() == "if" {
                Some(c.node)
            } else {
                None
            }
        })
        .map(|c| {
            format!(
                "Redundant if statement at line {} column {}",
                c.start_position().row + 1,
                c.start_position().column + 1
            )
        })
        .collect::<Vec<_>>();
    println!("{}", redundant_assignments.join("\n"));
}
