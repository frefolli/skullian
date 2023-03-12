use tree_sitter::Language;


/// Returns grammar: Option<tree_sitter::Language> from language name
/// 
/// # Arguments
/// 
/// * `language_name`: &str - the language name
/// 
/// # Examples
/// 
/// ```
/// let language_grammar = skullian::language::grammar::from_language_name("cpp");
/// ```
pub fn from_language_name(
    language_name: String
) -> Option<Language> {
    match language_name.as_str() {
        "java" => return Some(tree_sitter_java::language()),
        "c" => return Some(tree_sitter_c::language()),
        "cpp" => return Some(tree_sitter_cpp::language()),
        "rust" => return Some(tree_sitter_rust::language()),
        &_ => return None
    }
}

/// Returns grammar: Option<tree_sitter::Language> from file name
/// 
/// # Arguments
/// 
/// * `file_name`: &str - the file name
/// 
/// # Examples
/// 
/// ```
/// let language_grammar = skullian::language::grammar::from_file_name("main.cpp");
/// ```
pub fn from_file_name(
    file_name: &std::path::Path
) -> Option<Language> {
    match crate::language::name::from_file_name(file_name) {
        Some(language_name) => return from_language_name(language_name),
        None => return None
    }
}