# Skullian CLI

```
Usage:
  target\debug\skullian.exe [OPTIONS] [TARGETS ...]

Arcan DepGraph Generator with TS and SG

Positional arguments:
  targets               target files or directories

Optional arguments:
  -h,--help             Show this help message and exit
  -l,--language LANGUAGE
                        issue the language of targets
  -S,--stack-graph      perform stack graph action
  -T,--tree-sitter      perform tree sitter action
```

Tips:

- If `LANGUAGE` is not issued, it is inferred from `TARGET` file extension.
- If `TARGETS` is not issued it's defaulted to current directory.
- If a `TARGET` is a directory all of its content is recursively added as `TARGET`.