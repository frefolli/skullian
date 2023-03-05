/// build tree-sitter graph from file path and language name
/// 
/// # Arguments
/// 
/// * `file_path`: &str - the file name or path
/// * `language_name`: &str - the language name
/// 
/// # Behavior
/// 
/// * must return None when language_name is invalid
/// * must return None when language_name is unsupported
/// * must return None when file_name is invalid
/// * must return None when file is empty
/// * must return Some when syntax is invalid
pub fn from_file_name_and_language_name(
    file_path: &str,
    language_name: &str
) -> Option<tree_sitter::Tree> {
    let grammar = crate::language::grammar::from_language_name(language_name);
    if grammar.is_none() {
        return None
    }

    let text = std::fs::read_to_string(file_path);
    if text.is_err() || text.as_ref().unwrap().len() == 0 {
        return None;
    }
    
    let mut parser = tree_sitter::Parser::new();
    let result = parser.set_language(grammar.unwrap());

    if result.is_err() {
        panic!("{}", result.unwrap_err())
    }

    return parser.parse(text.unwrap(), None);
}

/// build tree-sitter graph from file name
/// detects language_name using file extension
/// 
/// # Arguments
/// 
/// * `file_path`: &str - the file name or path
/// 
/// # Behavior
/// 
/// * must return None when file extension is invalid
/// * must return None when language_name is unsupported
/// * must return None when file_name is invalid
/// * must return None when file is empty
/// * must return Some when syntax is invalid
pub fn from_file_name(
    file_path: &str
) -> Option<tree_sitter::Tree> {
    let grammar = crate::language::grammar::from_file_name(file_path);
    if grammar.is_none() {
        return None
    }

    let text = std::fs::read_to_string(file_path);
    if text.is_err() || text.as_ref().unwrap().len() == 0 {
        return None;
    }
    
    let mut parser = tree_sitter::Parser::new();
    let result = parser.set_language(grammar.unwrap());

    if result.is_err() {
        panic!("{}", result.unwrap_err())
    }

    return parser.parse(text.unwrap(), None);
}

/// build tree-sitter graph
/// 
/// # Arguments
/// 
/// * `text`: &str - the source code as text
/// * `language_name`: &str - the language name
/// 
/// # Behavior
/// 
/// * must return None when language_name is invalid
/// * must return None when language_name is unsupported
/// * must return None when text is empty
/// * must return Some when syntax is invalid
pub fn from_text_and_language_name(
    text: &str,
    language_name: &str
) -> Option<tree_sitter::Tree> {
    let grammar = crate::language::grammar::from_language_name(language_name);
    if grammar.is_none() {
        return None
    }
    
    if text.len() == 0 {
        return None;
    }
    
    let mut parser = tree_sitter::Parser::new();
    let result = parser.set_language(grammar.unwrap());

    if result.is_err() {
        panic!("{}", result.unwrap_err())
    }

    return parser.parse(text, None);
}

/// build tree-sitter graph
/// 
/// # Arguments
/// 
/// * `tree`: tree_sitter::Tree - the graph produced by tree-sitter
/// 
/// # Behavior
pub fn tree_to_sexp(
    tree: tree_sitter::Tree
) -> String {
    return tree.root_node().to_sexp();
}