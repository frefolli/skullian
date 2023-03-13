/// Returns tsg_path: Option<String> from language name
/// 
/// # Arguments
/// 
/// * `language_name`: &str - the language name
/// 
/// # Examples
/// 
/// ```
/// let tsg_path = skullian::language::tsg::from_language_name("java");
/// ```
pub fn from_language_name(
    language_name: &str
) -> Option<Box<std::path::Path>> {
    match language_name {
        "java" => return Some(std::path::Path::new("assets/tsg/java.tsg").into()),
        &_ => return None
    }
}

/// Returns tsg_path: Option<String> from file name
/// 
/// # Arguments
/// 
/// * `file_name`: &str - the file name
/// 
/// # Examples
/// 
/// ```
/// let tsg_path = skullian::language::tsg::from_file_name(
///     std::path::Path::new("main.java"));
/// ```
pub fn from_file_name(
    file_name: &std::path::Path
) -> Option<Box<std::path::Path>> {
    let language_name = crate::language::name::from_file_name(file_name).unwrap();
    crate::language::tsg::from_language_name(language_name)
}