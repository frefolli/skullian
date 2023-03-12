pub mod test_name_from_file_name {
    use skullian::language::name::from_file_name;

    #[test]
    pub fn test_with_supported_languages() {
        assert_eq!(
            Some("java".to_string()),
            from_file_name(std::path::Path::new("main.java")));
        assert_eq!(
            Some("c".to_string()),
            from_file_name(std::path::Path::new("main.c")));
        assert_eq!(
            Some("cpp".to_string()),
            from_file_name(std::path::Path::new("main.cpp")));
        assert_eq!(
            Some("cpp".to_string()),
            from_file_name(std::path::Path::new("main.cc")));
        assert_eq!(
            Some("rust".to_string()),
            from_file_name(std::path::Path::new("main.rs")));
    }
    
    #[test]
    pub fn test_with_unsupported_language() {
        assert_eq!(
            None,
            from_file_name(std::path::Path::new("main.foti")));
    }
}