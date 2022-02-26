# Linting example with tree-sitter
Minimal complete example of how to use the tree-sitter bindings to
perform some linting checks for the
[Imp](https://softwarefoundations.cis.upenn.edu/lf-current/Imp.html)
language.  See the file `src/main.rs` for more information.  It uses a
tree-sitter query to pattern-match over the AST, iterates through the
matches and reports them.

Given a file `factorial.imp` with contents

```
z := x;
y := 1;
y := y;
while ~(z=0) do
  y := y * z;
  z := z-1;
  x := x;
end;
x := x;
```

Running the following command

```ShellSession
$ cargo run -- factorial.imp
```

Produces the output

```
Redundant assignments:
Redundant assignment to y at row 2 column 0
Redundant assignment to x at row 6 column 2
Redundant assignment to x at row 8 column 0
```
