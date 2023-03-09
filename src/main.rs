use core::panic;
use skullian::{cli::CLIConfig, graph::sg::{ExtensionMethod, ExtendableWithTSGrammar}};

fn job_stack_graph(config: &CLIConfig) {
    println!("#----------------------------------------------------------------+job_stack_graph+----------------------------------------------------------------#");
    if config.file_name.is_empty() {
        panic!("no input file issued!");
    }

    let extension_method : ExtensionMethod;
    if config.language_name.is_empty() {
        if config.tsg_path.is_empty() {
            extension_method = ExtensionMethod::from_file_path(&config.file_name);
        } else {
            extension_method = ExtensionMethod::from_file_path_and_tsg_path(&config.file_name, &config.tsg_path);
        }
    } else {
        if config.tsg_path.is_empty() {
            extension_method = ExtensionMethod::from_file_path_and_language_name(&config.file_name, &config.language_name);
        } else {
            extension_method = ExtensionMethod::from_all(&config.file_name, &config.language_name, &config.tsg_path);
        }
    }

    let mut stack_graph = stack_graphs::graph::StackGraph::new();
    stack_graph.extend(&extension_method);
    skullian::graph::dg::walk_stack_graph(&stack_graph);
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

fn command_line() {
    let mut config = skullian::cli::new_empty_config();
    skullian::cli::parse_args(&mut config);

    if config.perform_job_ts {
        job_tree_sitter(&config);
    }

    if config.perform_job_sg {
        job_stack_graph(&config);
    }
}

fn main() {
    command_line();
    // skullian::graph::dg::main();
}
