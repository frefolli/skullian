/// returns language_name: &str from file name
/// by looking at its extension
///
/// # Arguments
///
/// * `file_name`: &str - the file name or path
/// 
/// # Examples
/// 
/// ```
/// let language_name = skullian::language::name::from_file_name("/home/user/Documents/main.cpp");
/// ```
pub fn from_file_name(
    file_name: &std::path::Path
) -> Option<String> {
    let extension = std::path::Path::new(file_name).extension().unwrap_or_default().to_str().unwrap();
    match extension {
        "java" => return Some("java".to_string()),
        "c" => return Some("c".to_string()),
        "cpp" => return Some("cpp".to_string()),
        "cc" => return Some("cpp".to_string()),
        "rs" => return Some("rust".to_string()),
        &_ => return None
    }
}