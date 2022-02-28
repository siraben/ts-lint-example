# Linting example with tree-sitter
Minimal complete example of how to use the tree-sitter bindings to
perform some linting checks for the
[Imp](https://softwarefoundations.cis.upenn.edu/lf-current/Imp.html)
language.  See the file `src/main.rs` for more information.  It uses a
tree-sitter query to pattern-match over the AST, iterates through the
matches and reports them.  Planned checks include:

- [x] redundant assignments
- [x] redundant if statement
- [ ] always false condition

Since Imp has a simple operational semantics, it can be easily proven
that these suggestions preserve program behavior.

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
if x = y then x := 1 else x := 1 end
```

Running the following command

```ShellSession
$ cargo run -- factorial.imp
```

Produces the output

```
Redundant assignments:
Redundant assignment to y at line 3 column 1
Redundant assignment to x at line 7 column 3
Redundant assignment to x at line 9 column 1
Redundant if statement:
Redundant if statement at line 10 column 1
```
