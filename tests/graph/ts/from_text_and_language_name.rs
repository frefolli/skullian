use skullian::graph::ts::from_text_and_language_name;

#[test]
pub fn test_with_invalid_language_name() {
    let tree = from_text_and_language_name(
        "".to_string(),
        "".to_string());
    assert!(tree.is_none());
}

#[test]
pub fn test_with_unsupported_language_name() {
    let tree = from_text_and_language_name(
        "".to_string(),
        "foti".to_string());
    assert!(tree.is_none());
}

#[test]
pub fn test_with_empty_text() {
    let tree = from_text_and_language_name(
        "".to_string(),
        "cpp".to_string());
    assert!(tree.is_none());
}

#[test]
pub fn test_with_invalid_syntax() {
    let tree = from_text_and_language_name(
        "int main() { bing_chilling }".to_string(),
        "cpp".to_string());
    assert!(tree.is_some());
}

#[test]
pub fn test_with_all_ok() {
    let tree = from_text_and_language_name(
        "#include<iostream>\nint main() { std::cout << 1 << std::endl; }".to_string(),
        "cpp".to_string());
    assert!(tree.is_some());
}