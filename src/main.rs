use core::panic;
use std::ops::Index;
use skullian::cli::CLIConfig;

fn job_stack_graph(config: &CLIConfig) {
    println!("#----------------------------------------------------------------+job_stack_graph+----------------------------------------------------------------#");
    let source_code = std::fs::read_to_string(&config.file_name).expect("no inputs file issued");
    let grammar: Option<tree_sitter::Language>;
    if config.language_name.is_empty() {
        let language_name = skullian::language::name::from_file_name(config.file_name.as_str());
        if language_name.is_none() {
            panic!("unable to detect a language name from file name");
        }
        grammar = skullian::language::grammar::from_language_name(language_name.unwrap());
        if grammar.is_none() {
            panic!("language is not supported");
        }
    } else {
        grammar = skullian::language::grammar::from_language_name(config.language_name.as_str());
        if grammar.is_none() {
            panic!("language is not supported");
        }
    }
    let ts_rules = std::fs::read_to_string(&config.stack_graph_rules).expect("stack graph rules not issued");
    let language = tree_sitter_stack_graphs::StackGraphLanguage::from_str(grammar.unwrap(), ts_rules.as_str()).unwrap();
    let mut stack_graph = stack_graphs::graph::StackGraph::new();
    let file_handle = stack_graph.get_or_create_file(&config.file_name);
    let globals = tree_sitter_stack_graphs::Variables::new();
    match language.build_stack_graph_into(
        &mut stack_graph,
        file_handle,
        source_code.as_str(),
        &globals,
        &tree_sitter_stack_graphs::NoCancellation
    ) {
        Ok(()) => (),
        Err(_err) => {
            println!("{}", _err);
            panic!();
        }
    };

    for node_handle in stack_graph.iter_nodes() {
        let node = stack_graph.index(node_handle);
        if node.symbol().is_some() {
            println!("(node \"{}\")", stack_graph.index(node.symbol().unwrap()));
        }
    }
        let mut paths = stack_graphs::paths::Paths::new();
        paths.find_all_paths(
            &stack_graph,
            stack_graph.iter_nodes(),
            &stack_graphs::NoCancellation,
            |_the_stack_graph, _the_paths, _the_path| {
                if _the_path.is_complete(&_the_stack_graph) {
                    print!("(path");
                    for edge in _the_path.edges.iter(_the_paths) {
                        let node = stack_graph.index(stack_graph.node_for_id(edge.source_node_id).unwrap());
                        if node.is_definition() {
                            if node.symbol().is_some() {
                                print!(" {}", stack_graph.index(node.symbol().unwrap()))
                            }
                        }
                    }
                    let node = stack_graph.index(_the_path.end_node);
                    if node.is_definition() {
                        if node.symbol().is_some() {
                            print!(" {}", stack_graph.index(node.symbol().unwrap()))
                        }
                    }
                    println!(")");
                }
            }
        ).unwrap();
    println!("#----------------------------------------------------------------!job_stack_graph!----------------------------------------------------------------#");
}

fn job_tree_sitter(config: &CLIConfig) {
    println!("#----------------------------------------------------------------+job_tree_sitter+----------------------------------------------------------------#");
    if config.file_name.is_empty() {
        panic!("no input file issued!");
    } else {
        let file_name = config.file_name.clone();
        let tree: Option<tree_sitter::Tree>;
        if config.language_name.is_empty() {
            tree = skullian::graph::ts::from_file_name(file_name.as_str());
        } else {
            tree = skullian::graph::ts::from_file_name_and_language_name(
                file_name.as_str(),
                config.language_name.as_str());
        }
        if tree.is_none() {
            panic!("error while parsing file {}", file_name)
        } else {
            println!("{}", skullian::graph::ts::tree_to_sexp(tree.unwrap()))
        }
    }
    println!("#----------------------------------------------------------------!job_tree_sitter!----------------------------------------------------------------#");
}

fn main() {
    let mut config = skullian::cli::new_empty_config();
    skullian::cli::parse_args(&mut config);

    if config.perform_job_ts {
        job_tree_sitter(&config);
    }

    if config.perform_job_sg {
        job_stack_graph(&config);
    }
}
