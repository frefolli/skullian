pub mod test_name_from_file_name {
    use skullian::language::name::from_file_name;

    #[test]
    pub fn test_with_supported_languages() {
        assert_eq!(Some("java"), from_file_name("main.java"));
        assert_eq!(Some("c"), from_file_name("main.c"));
        assert_eq!(Some("cpp"), from_file_name("main.cpp"));
        assert_eq!(Some("cpp"), from_file_name("main.cc"));
        assert_eq!(Some("rust"), from_file_name("main.rs"));
    }
    
    #[test]
    pub fn test_with_unsupported_language() {
        assert_eq!(None, from_file_name("main.foti"))
    }
}