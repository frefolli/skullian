use core::panic;
use std::path::Path;
use skullian::{cli::CLIConfig, graph::sg::{ExtensionMethod, ExtendableWithTSGrammar}};

trait PathProcessor : Sized {
    fn process(&mut self, path_str : &str);
}

fn map_target_files<P : PathProcessor>(config: &CLIConfig, processor: &mut P) {
    for target in &config.targets {
        let target_path = Path::new(target);
        if !target_path.exists() {
            panic!("target {} doesn't exists", target_path.display());
        }
        for direntry in walkdir::WalkDir::new(target_path) {
            let entry = direntry.unwrap();
            let path_str = entry.path().to_str().unwrap();
            if entry.file_type().is_file() {
                processor.process(path_str);
            }
        }
    }
}

struct TreeSitterProcessor<'a> {
    config: &'a CLIConfig
}

impl <'a> PathProcessor for TreeSitterProcessor<'a> {
    fn process(&mut self, path_str : &str) {
        let tree: Option<tree_sitter::Tree>;
        if self.config.language_name.is_empty() {
            tree = skullian::graph::ts::from_file_name(path_str);
        } else {
            tree = skullian::graph::ts::from_file_name_and_language_name(
                path_str,
                self.config.language_name.as_str());
        }
        if tree.is_none() {
            panic!("error while parsing file {}", path_str)
        } else {
            println!("# --- {} --- #", path_str);
            println!("{}",
                skullian::graph::ts::tree_to_sexp(tree.unwrap()));
            println!("# --- {} --- #", path_str);
        }
    }
}

impl <'a> TreeSitterProcessor<'a> {
    pub fn new(config: &CLIConfig) -> TreeSitterProcessor {
        TreeSitterProcessor {
            config: config
        }
    }
}

struct StackGraphProcessor<'a> {
    config: &'a CLIConfig,
    stack_graph: &'a mut stack_graphs::graph::StackGraph,
    globals: &'a mut tree_sitter_stack_graphs::Variables<'a>
}

impl <'a> StackGraphProcessor<'a> {
    pub fn new(config: &'a CLIConfig,
               stack_graph: &'a mut stack_graphs::graph::StackGraph,
               globals: &'a mut tree_sitter_stack_graphs::Variables<'a>) -> StackGraphProcessor<'a> {
        StackGraphProcessor {
            config,
            stack_graph: stack_graph,
            globals: globals
        }
    }
}

impl <'a> PathProcessor for StackGraphProcessor<'a> {
    fn process(&mut self, path_str : &str) {
        let file_name = String::from(path_str);
        let extension_method : ExtensionMethod;
        if self.config.language_name.is_empty() {
            extension_method = ExtensionMethod::from_file_path(&file_name);
        } else {
            extension_method = ExtensionMethod::from_file_path_and_language_name(&file_name, &self.config.language_name);
        }
        self.stack_graph.extend(self.globals, &extension_method);
    }
}

fn job_stack_graph(config: &CLIConfig) {
    let mut stack_graph = stack_graphs::graph::StackGraph::new();
    let mut globals = tree_sitter_stack_graphs::Variables::new();
    let mut processor = StackGraphProcessor::new(config, &mut stack_graph, &mut globals);
    map_target_files(config, &mut processor);
    skullian::graph::dg::walk_stack_graph(&stack_graph);
}


fn job_tree_sitter(config: &CLIConfig) {
    let mut processor = TreeSitterProcessor::new(config);
    map_target_files(config, &mut processor);
}

fn command_line() {
    let mut config = skullian::cli::CLIConfig::new_empty();
    skullian::cli::parse_args(&mut config);
    config.derive_action();
    
    match config.action {
        skullian::cli::CLIAction::TreeSitter() => job_tree_sitter(&config),
        skullian::cli::CLIAction::StackGraph() => job_stack_graph(&config),
        _ => ()
    }
}

fn main() {
    command_line();
}
