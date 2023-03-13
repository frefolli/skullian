use stack_graphs::graph::StackGraph;
use tree_sitter_stack_graphs::{LanguageError, StackGraphLanguage};

pub struct ExtensionMethod {
    file_path: Box<std::path::Path>,
    language_name: String,
    tsg_path: Box<std::path::Path>
}

impl ExtensionMethod {
    pub fn from_file_path(file_path: &std::path::Path) -> ExtensionMethod {
        ExtensionMethod {
            file_path: file_path.into(),
            language_name: crate::language::name::from_file_name(file_path).expect("unable to load language_name"),
            tsg_path: crate::language::tsg::from_file_name(file_path).expect("unable to load tsg_path"),
        }
    }

    pub fn from_file_path_and_tsg_path(file_path: &std::path::Path, tsg_path: &std::path::Path) -> ExtensionMethod {
        ExtensionMethod {
            file_path: file_path.into(),
            language_name: crate::language::name::from_file_name(file_path).expect("unable to load language_name"),
            tsg_path: tsg_path.into()
        }
    }

    pub fn from_file_path_and_language_name(file_path: &std::path::Path, language_name: String) -> ExtensionMethod {
        ExtensionMethod {
            file_path: file_path.into(),
            language_name: language_name.clone(),
            tsg_path: crate::language::tsg::from_file_name(file_path).expect("unable to load tsg_path")
        }
    }

    pub fn from_all(file_path: &std::path::Path, language_name: &String, tsg_path: &std::path::Path) -> ExtensionMethod {
        ExtensionMethod {
            file_path: file_path.into(),
            language_name: language_name.clone(),
            tsg_path: tsg_path.into()
        }
    }

    pub fn as_stack_graph_language(&self) -> Result<StackGraphLanguage, LanguageError> {
        let grammar = crate::language::grammar::from_language_name(self.language_name.clone()).expect("unable to load language_grammar");
        let ts_rules = std::fs::read_to_string(&self.tsg_path).expect("stack graph rules not issued");
        tree_sitter_stack_graphs::StackGraphLanguage::from_str(grammar, ts_rules.as_str())
    }
}

pub trait ExtendableWithTSGrammar {
    fn extend(&mut self, globals: &mut tree_sitter_stack_graphs::Variables, extension_method: &ExtensionMethod);
}

impl ExtendableWithTSGrammar for StackGraph {
    fn extend(&mut self, globals: &mut tree_sitter_stack_graphs::Variables, extension_method: &ExtensionMethod) {
        let source_code = std::fs::read_to_string(&extension_method.file_path).expect("no inputs file issued");
        log::info!("StackGraph is_done_with loading source_code");
        let stack_graph_language = extension_method.as_stack_graph_language().expect("unable to load stack_graph_language");
        log::info!("StackGraph is_done_with loading sgl");
        let file_handle = self.get_or_create_file(extension_method.file_path.as_os_str().to_str().unwrap());
        log::info!("StackGraph is_done_with loading file handle");
        stack_graph_language.build_stack_graph_into(
            self,
            file_handle,
            source_code.as_str(),
            globals,
            &tree_sitter_stack_graphs::NoCancellation
        ).expect("unable to load stack_graph");
    }
}
