pub mod test_from_language_name {
    use skullian::language::grammar::from_language_name;

    #[test]
    pub fn test_with_supported_languages() {
        assert_eq!(
            Some(tree_sitter_java::language()),
            from_language_name("java".to_string()));
        assert_eq!(
            Some(tree_sitter_c::language()),
            from_language_name("c".to_string()));
        assert_eq!(
            Some(tree_sitter_cpp::language()),
            from_language_name("cpp".to_string()));
        assert_eq!(
            Some(tree_sitter_rust::language()),
            from_language_name("rust".to_string()));
    }
    
    #[test]
    pub fn test_with_unsupported_language() {
        assert_eq!(None, from_language_name("foti".to_string()));
    }
}