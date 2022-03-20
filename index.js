import Parser from "tree-sitter";
import Imp from "tree-sitter-imp";
import { readFileSync } from "fs";
const { Query } = Parser;

const parser = new Parser();
parser.setLanguage(Imp);

// Load the file factorial.imp
const sourceCode = readFileSync("factorial.imp", "utf8");
const tree = parser.parse(sourceCode);

// Query for redundant assignments
const redundantQuery = new Query(
  Imp,
  "((asgn name: (id) @left _ @right) (#eq? @left @right)) @redundantAsgn"
);

// Given a raw list of captures, extract the row, column and text.
function formatCaptures(tree, captures) {
  return captures.map((c) => {
    const node = c.node;
    delete c.node;
    c.text = tree.getText(node);
    c.row = node.startPosition.row;
    c.column = node.startPosition.column;
    return c;
  });
}

// Get the captures corresponding to a capture name
function capturesByName(tree, query, name) {
  return formatCaptures(
    tree,
    query.captures(tree.rootNode).filter((x) => x.name == name)
  ).map(function (x) {
    delete x.name;
    return x;
  });
}

// Lint the tree with a given message, query and match name
function lint(tree, msg, query, name) {
  console.log(msg);
  console.log(capturesByName(tree, query, name));
}

lint(tree, "Redundant assignments:", redundantQuery, "redundantAsgn");

// Query for redundant if branches
const redundantIfQuery = new Query(
  Imp,
  "((if condition: _ @c consequent: _ @l alternative: _ @r) (#eq? @l @r)) @redundantIf"
);

lint(tree, "Redundant if statements:", redundantIfQuery, "redundantIf");
