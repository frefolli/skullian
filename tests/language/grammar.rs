pub mod get_grammar_from_language_name {
    use skullian::language::grammar::from_language_name;

    #[test]
    pub fn test_with_supported_languages() {
        assert_eq!(Some(tree_sitter_java::language()), from_language_name("java"));
        assert_eq!(Some(tree_sitter_c::language()), from_language_name("c"));
        assert_eq!(Some(tree_sitter_cpp::language()), from_language_name("cpp"));
        assert_eq!( Some(tree_sitter_rust::language()), from_language_name("rust"));
    }
    
    #[test]
    pub fn test_with_unsupported_language() {
        assert_eq!(None, from_language_name("foti"));
    }
}