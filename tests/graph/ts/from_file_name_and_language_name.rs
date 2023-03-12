use skullian::graph::ts::from_file_name_and_language_name;
#[test]
pub fn test_with_invalid_language_name() {
    let tree = from_file_name_and_language_name(
        std::path::Path::new("tests/graph/ts/main.cpp"),
        "".to_string());
    assert!(tree.is_none());
}
#[test]
pub fn test_with_unsupported_language_name() {
    let tree = from_file_name_and_language_name(
        std::path::Path::new("tests/graph/ts/main.cpp"),
        "foti".to_string());
    assert!(tree.is_none());
}
#[test]
pub fn test_with_invalid_file_name() {
    let tree = from_file_name_and_language_name(
        std::path::Path::new("tests/graph/ts/not_found.cpp"),
        "cpp".to_string());
    assert!(tree.is_none());
}
#[test]
pub fn test_with_empty_file() {
    let tree = from_file_name_and_language_name(
        std::path::Path::new("tests/graph/ts/empty_file.cpp"),
        "cpp".to_string());
    assert!(tree.is_none());
}
#[test]
pub fn test_with_invalid_syntax() {
    let tree = from_file_name_and_language_name(
        std::path::Path::new("tests/graph/ts/invalid_syntax.cpp"),
        "cpp".to_string());
    assert!(tree.is_some());
}
#[test]
pub fn test_with_all_ok() {
    let tree = from_file_name_and_language_name(
        std::path::Path::new("tests/graph/ts/ok.cpp"),
        "cpp".to_string());
    assert!(tree.is_some());
}