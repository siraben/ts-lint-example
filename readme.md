# Linting example with tree-sitter
Minimal complete example of how to use the tree-sitter bindings to
perform some linting checks for the
[Imp](https://softwarefoundations.cis.upenn.edu/lf-current/Imp.html)
language.  See the file `index.js` for more information.  It uses a
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
while ~(z = 0) do
  y := y * z;
  z := z - 1;
  x := x;
end;
x := x;
if x = y then x := 1 else x := 1 end
```

Running the following command

```ShellSession
$ node index.js factorial.imp
```

Produces the output

```
Redundant assignments:
[
  { text: 'y := y', row: 2, column: 0 },
  { text: 'x := x', row: 6, column: 2 },
  { text: 'x := x', row: 8, column: 0 }
]
Redundant if statements:
[ { text: 'if x = y then x := 1 else x := 1 end', row: 9, column: 0 } ]
```
