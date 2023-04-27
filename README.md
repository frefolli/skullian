# Skullian

PoC DepGraph Generator with Tree Sitter and Stack Graphs

Goto [md documentation](docs/README.md) or [hosted documentation](https://frefolli.github.io/skullian/skullian/index.html)

# TODO

```
    priority = [0, +inf)
    higher priority = later done
```

| activity | priority | status |
| -------- | -------- | ------ |
| add castsType edge and refkind | 0 | - [] |
| add throwsType edge and refkind | 0 | - [] |
| adapt TSG to use `castsType` with type casts | 1 | - [] |
| adapt TSG to use `throwsType` with exception throwal | 2 | - [] |
| adapt TSG to use `usesType` with annotation usage | 3 | - [] |
| adapt TSG to use `extension` as bridge for name resolution | 4 | - [] |
| adapt TSG to use `implementation` as bridge for name resolution | 5 | - [] |
| adapt TSG to use `typing` as bridge for name resolution | 6 | - [] |
| add xml serialization for DepGraph targetting GraphML | 7 | - [] |
|  |  |  |
