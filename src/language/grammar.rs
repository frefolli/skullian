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
    language_name: &str
) -> Option<Language> {
    match language_name {
        "java" => return Some(tree_sitter_java::language()),
        "c" => return Some(tree_sitter_c::language()),
        "cpp" => return Some(tree_sitter_cpp::language()),
        "rust" => return Some(tree_sitter_rust::language()),
        &_ => return None
    }
}