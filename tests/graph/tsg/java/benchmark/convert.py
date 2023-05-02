def read_csvstring(string: str) -> list[list]:
    lines = string.strip().split("\n")
    rows = [[_.strip() for _ in line.split(",")] for line in lines]
    return rows

def read_csvfile(filepath: str) -> list[list]:
    with open(filepath, mode="r", encoding="utf-8") as file:
        return read_csvstring(file.read())

def forge_test_yml(files, rels):
    ss = "filepaths:"
    ss += "".join(["\n  - " + _[0] for _ in files[1:]])
    ss += "\nnodes: []"
    ss += "\nedges:"
    for row in rels[1:]:
        ss += "\n#   testid: " + row[0]
        ss += "\n  - source: " + row[1]
        ss += "\n    sink: " + row[2]
        ss += "\n    kind: " + row[3]
    return ss

if __name__ == "__main__":
    files = read_csvfile("files.csv")
    rels = read_csvfile("manifest.csv")
    yml = forge_test_yml(files, rels)
    with open("test.yml", mode="w", encoding="utf-8") as file:
        file.write(yml)
