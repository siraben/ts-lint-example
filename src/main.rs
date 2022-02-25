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
    let query_str = "((asgn name: (id) @left _ @right) (#eq? @left @right)) @red";
    let query = Query::new(language, query_str).unwrap();

    println!("Matches:");
    let mut query_cursor = QueryCursor::new();
    let source = std::fs::read_to_string(file).unwrap();
    let matches = query_cursor.captures(&query, tree.root_node(), source.as_bytes());
    let types: Vec<(_, _)> = matches
        .flat_map(|(m, _)| m.captures)
        .filter_map(|c| c.node.child_by_field_name("name"))
        .map(|c| {
            (
                source[c.byte_range()].to_string(),
                (c.start_position().row, c.start_position().column),
            )
        })
        .collect();
    // .filter_map(|c| c.node.child_by_field_name("name"))
    // .map(|id_node| ( source[id_node.byte_range()].to_string(), id_node.start_position().row, ))
    // .collect();
    println!("{:?}", types);

    // for match_ in matches {
    //     println!("  {:?} (pattern_index={})", match_, match_.pattern_index);
    //     for capture in match_.captures {
    //         let node = capture.node;
    //         println!("    {:?} (index={})", node, capture.index);
    //     }
    // }
    // qc.matches(q, r, r);
    // println!("{}",q.start_byte_for_pattern(0));
    // for i in q.capture_names() {
    //     println!("{}",i)
    // }
    // println!("Parse tree for {}", file);
    // print!(
    //     "{}",
    //     r.to_sexp()
    // );
}
