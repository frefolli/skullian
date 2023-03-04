pub fn get_language_name_from_file_name(
    file_name: &str
) -> Option<&str> {
    let extension = std::path::Path::new(file_name).extension().unwrap_or_default().to_str().unwrap();
    match extension {
        "java" => return Some("java"),
        "c" => return Some("c"),
        "cpp" => return Some("cpp"),
        "cc" => return Some("cpp"),
        "rs" => return Some("rust"),
        &_ => return None
    }
}