use tree_sitter::Language;

pub fn get_grammar_from_language_name(
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