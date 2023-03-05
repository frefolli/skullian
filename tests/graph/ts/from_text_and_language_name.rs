use skullian::graph::ts::from_text_and_language_name;

#[test]
pub fn test_with_invalid_language_name() {
    let tree = from_text_and_language_name(
        "",
        "");
    assert!(tree.is_none());
}

#[test]
pub fn test_with_unsupported_language_name() {
    let tree = from_text_and_language_name(
        "",
        "foti");
    assert!(tree.is_none());
}

#[test]
pub fn test_with_empty_text() {
    let tree = from_text_and_language_name(
        "",
        "cpp");
    assert!(tree.is_none());
}

#[test]
pub fn test_with_invalid_syntax() {
    let tree = from_text_and_language_name(
        "int main() { bing_chilling }",
        "cpp");
    assert!(tree.is_some());
}

#[test]
pub fn test_with_all_ok() {
    let tree = from_text_and_language_name(
        "#include<iostream>\nint main() { std::cout << 1 << std::endl; }",
        "cpp");
    assert!(tree.is_some());
}