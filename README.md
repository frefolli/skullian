# Skullian

PoC DepGraph Generator with Tree Sitter and Stack Graphs

Goto [md documentation](docs/README.md) or [hosted documentation](https://frefolli.github.io/skullian/skullian/index.html)

## TODO

```
    priority = [0, +inf)
    higher priority = later done
```

| activity | priority | status |
| -------- | -------- | ------ |
| add castsType edge and refkind | 0 | &check; |
| add throwsType edge and refkind | 0 | &check; |
| adapt TSG to use `castsType` with type casts | 1 | &check; |
| adapt TSG to use `throwsType` with exception throwal | 2 | &check; |
| adapt TSG to use `usesType` with object creation | 3 | &check; |
| adapt TSG to use `usesType` with array creation | 4 | &check; |
| adapt TSG to use `usesType` with annotation usage | 5 | &check; |
| adapt TSG to use `typing` as bridge for name resolution | 6 | &check; |
| adapt TSG to use `extension` as bridge for name resolution | 7 | &check; |
| adapt TSG to use `implementation` as bridge for name resolution | 8 | &check; |
| enable passthrough for function call parameters | 9 | &check; |
| write a test for prujit | 10 | &check; |
| edit test framework to print all errors instead of just one | 11 | &check; |
| locate error source of prujit | 11 | &cross; |
| attempt to remove cycles from TSG | 12 | &cross; |
| optimize cycle detector | 13 | &cross; |
| add xml serialization for DepGraph targetting GraphML | 14 | &cross; |
|  |  |  |

## Quality Reports

- [Quality Report for Java](report_java.md)
- [Quality Report for C++](report_cpp.md)
- [Quality Report for Python](report_python.md)
