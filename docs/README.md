# Skullian Documentation

Index:

- [building](#building)
- [running](#running)
- [testing](#testing)
- [packaging](#packaging)
- [state of art](#state-of-art)

# Building

run either:

- ```cargo build``` to build the debug executable
- ```cargo build -r``` to build the release executable

# Running

run either:

- ```cargo run-- arguments ...``` to run the debug executable
- ```cargo run -r -- arguments ...``` to run the release executable

Where `arguments` is the list of args you want to pass.

With `cargo run -r -- -h`:

```txt
Usage:
  target\release\skullian.exe [OPTIONS] [TARGETS ...]

Arcan DepGraph Generator with TS and SG

Positional arguments:
  targets               target files or directories

Optional arguments:
  -h,--help             Show this help message and exit
  -f,--extension EXTENSION
                        filter files by extension
  -l,--language LANGUAGE
                        issue the language of targets
  -o,--output-file OUTPUT_FILE
                        issue the path of output file (default = ./data.json)
  -S,--stack-graph      perform stack graph action
  -D,--debug            perform debug action
  -W,--workflow         perform workflow action
  -T,--tree-sitter      perform tree sitter action
  -v,--verbose          if verbose LogLevel = INFO, else LogLevel = WARN

- If LANGUAGE is not issued, it is inferred from TARGET file extension.
- If TARGETS is not issued it's defaulted to current directory.
- If a TARGET is a directory all of its content is recursively added as TARGET.
```

## Tree Sitter Action

This action builds Tree Sitter AST from each source code file.
Doesn't fail if a syntax error happen (tree sitter reports it in the tree).

## Stack Graph Action

This action is thought for testing a .tsg file against Stack Graph generation with Tree Sitter.

## Debug Action

This action is thought for testing a .tsg file against Dep Graph generation with Tree Sitter and Stack Graph. Currently i'm rewriting the java tsg using this action.

`cargo run -r -- -D tests/graph/tsg/java` ensures features get detected correctly.

## Workflow Action

This is the action that contains the proof-of-concept workflow of skullian tool:

- builds stack graphs using .tsg files
- walks from stack graph root node and build a dep graph
- saves the dep graph as json in cytoscape graph format

# Testing

run either:

- ```cargo test``` to run tests on debug executable
- ```cargo test -r``` to run tests on release executable

Will perform unit tests and doctests

To run also tsg tests:

- ```cargo run -- -D .\tests\graph\tsg``` to run tests on debug executable
- ```cargo run -r -- -D .\tests\graph\tsg``` to run tests on release executable

# Packaging

## Debian

Using `cargo-deb` crate (install it with `cargo install cargo-deb`), use `cargo deb` from root folder. It'll create a deb package inside `target/`.

# State of Art

## Categories

| category | note | java | cpp |
| --- | --- | --- |
| file |  | &cross; | &check; |
| import |  | &check; | &check; |
| package |  | &check; | &check; |
| class |  | &check; | &check; |
| interface |  | &check; |  |
| method |  | &check; | &check; |
| parameter |  | &check; | &check; |
| attribute |  | &check; | &check; |

## Relationships

| relationship | note | java | cpp |
| --- | --- | --- |
| definedBy |  | &check; | &check; |
| nestedTo |  | &check; | &check; |
| isChildOf |  | &check; | &check; |
| isImplementationOf |  | &check; |  |
| includes |  | &check; | &check; |
| accessField |  | &check; | &cross; |
| usesType |  | &check; | &check; |
| calls |  | &check; | &cross; |
