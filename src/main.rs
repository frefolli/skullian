fn prova() {
    use stack_graphs::graph::Node::Root;
    // testing
    let source_code = r#"
    #include <iostream>
    void print_num(int s = 0) {
        std::cout << s << std::endl;
    }
    int main(int argc, char** args) {
        print_num(1);
        print_num(2);
        print_num(3);
    }
    "#;
    let grammar = skullian::language::grammar::from_language_name("cpp").unwrap();
    let ts_rules = r#"
    "#;
    let language = tree_sitter_stack_graphs::StackGraphLanguage::from_str(grammar, ts_rules).unwrap();
    let mut stack_graph = stack_graphs::graph::StackGraph::new();
    let file_handle = stack_graph.get_or_create_file("test.py");
    let globals = tree_sitter_stack_graphs::Variables::new();
    language.build_stack_graph_into(&mut stack_graph, file_handle, source_code, &globals, &tree_sitter_stack_graphs::NoCancellation).unwrap();
}

fn workflow() {
    let mut config = skullian::cli::new_empty_config();
    skullian::cli::parse_args(&mut config);
    println!("file_name: {:?}", config.file_names);
    println!("language_name: {}", config.language_name);
    for file_name in config.file_names {
        let tree = skullian::graph::ts::from_file_name_and_language_name(
            file_name.as_str(),
            config.language_name.as_str());
        if tree.is_none() {
            panic!("error while parsing file {}", file_name)
        } else {
            println!("{}", skullian::graph::ts::tree_to_sexp(tree.unwrap()))
        }
    }
}

fn main() {
    // prova();
    workflow();
}