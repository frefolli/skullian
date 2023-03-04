pub mod get_language_name_from_file_name {
    use crate::language::recognition::get_language_name_from_file_name;

    #[test]
    pub fn test_with_supported_languages() {
        assert_eq!(Some("java"), get_language_name_from_file_name("main.java"));
        assert_eq!(Some("c"), get_language_name_from_file_name("main.c"));
        assert_eq!(Some("cpp"), get_language_name_from_file_name("main.cpp"));
        assert_eq!(Some("cpp"), get_language_name_from_file_name("main.cc"));
        assert_eq!(Some("rust"), get_language_name_from_file_name("main.rs"));
    }
    
    #[test]
    pub fn test_with_unsupported_language() {
        assert_eq!(None, get_language_name_from_file_name("main.foti"))
    }
}