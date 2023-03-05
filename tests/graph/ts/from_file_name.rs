use skullian::graph::ts::from_file_name;
#[test]
pub fn test_with_invalid_file_extension() {
    let tree = from_file_name(
        "examples/tests/graph/ts/main");
    assert!(tree.is_none());
}
#[test]
pub fn test_with_unsupported_language_name() {
    let tree = from_file_name(
        "examples/tests/graph/ts/main.foti");
    assert!(tree.is_none());
}
#[test]
pub fn test_with_invalid_file_name() {
    let tree = from_file_name(
        "examples/tests/graph/ts/not_found.cpp");
    assert!(tree.is_none());
}
#[test]
pub fn test_with_empty_file() {
    let tree = from_file_name(
        "examples/tests/graph/ts/empty_file.cpp");
    assert!(tree.is_none());
}
#[test]
pub fn test_with_invalid_syntax() {
    let tree = from_file_name(
        "examples/tests/graph/ts/invalid_syntax.cpp");
    assert!(tree.is_none());
}
#[test]
pub fn test_with_all_ok() {
    let tree = from_file_name(
        "examples/tests/graph/ts/ok.cpp");
    assert!(tree.is_some());
}