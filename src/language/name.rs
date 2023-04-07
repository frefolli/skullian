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
/// let language_name = skullian::language::name::from_file_name(
///     std::path::Path::new("/home/user/Documents/main.cpp"));
/// ```
pub fn from_file_name(
    file_name: &std::path::Path
) -> Option<&str> {
    let extension = std::path::Path::new(file_name).extension().unwrap_or_default().to_str().unwrap();
    match extension {
        "java" => return Some("java"),
        "c" => return Some("c"),
        "cpp" => return Some("cpp"),
        "cc" => return Some("cpp"),
        "rs" => return Some("rust"),
        "py" => return Some("python"),
        &_ => return None
    }
}