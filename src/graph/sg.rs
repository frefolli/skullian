use stack_graphs::graph::StackGraph;
use tree_sitter_stack_graphs::StackGraphLanguage;

pub trait ExtendableWithTSGrammar {
    fn extend(
        &mut self,
        globals: &mut tree_sitter_stack_graphs::Variables,
        file_path: &std::path::Path,
        stack_graph_language: &StackGraphLanguage);
}

impl ExtendableWithTSGrammar for StackGraph {
    fn extend(
        &mut self,
        globals: &mut tree_sitter_stack_graphs::Variables,
        file_path: &std::path::Path,
        stack_graph_language: &StackGraphLanguage
    ) {
        let source_code = std::fs::read_to_string(file_path).expect("no inputs file issued");
        let file_handle = self.get_or_create_file(file_path.as_os_str().to_str().unwrap());
        match stack_graph_language.build_stack_graph_into(
            self,
            file_handle,
            source_code.as_str(),
            globals,
            &tree_sitter_stack_graphs::NoCancellation
        ) {
            Err(e) => log::info!("{}",e),
            //Err(e) => panic!("{}",e),
            _ => ()
        }
    }
}
