use core::panic;
use std::{path::Path, collections::HashMap, ops::Index};
use log::LevelFilter;
use log4rs::{append::console::ConsoleAppender, Config, config::{Appender, Root}};
use skullian::{cli::CLIConfig, graph::{sg::ExtendableWithTSGrammar, dg::{testing::TestCase, dep_graph::DepGraph}}};
use stack_graphs::graph::StackGraph;
use tree_sitter_stack_graphs::{StackGraphLanguage, Variables};

fn tree_sitter_process(config: &CLIConfig, path_str : &std::path::Path) {
    let tree: Option<tree_sitter::Tree>;
    if config.language_name.is_empty() {
        tree = skullian::graph::ts::from_file_name(path_str);
    } else {
        tree = skullian::graph::ts::from_file_name_and_language_name(
            path_str,
            &config.language_name);
    }
    if tree.is_none() {
        panic!("error while parsing file {}", path_str.display())
    } else {
        log::info!("# --- {} --- #", path_str.display());
        log::info!("{}", skullian::graph::ts::tree_to_sexp(&tree.unwrap()));
        log::info!("# --- {} --- #", path_str.display());
    }
    log::info!("TreeSitterProcessor is_done_with {}", path_str.display());
}

fn job_tree_sitter(config: &CLIConfig) {
    for target in &config.targets {
        let target_path = Path::new(target);
        if !target_path.exists() {
            panic!("target {} doesn't exists", target_path.display());
        }
        for direntry in walkdir::WalkDir::new(target_path) {
            let entry = direntry.unwrap();
            let extension = entry.path().extension().unwrap_or_default().to_str().unwrap().to_string();
            if entry.file_type().is_file() {
                if
                    config.file_extension.is_empty() ||
                    (extension.len() != 0 &&
                    config.file_extension == extension)
                {
                    tree_sitter_process(config, entry.path());
                }
            }
        }
    }
}

fn stack_graph_process(
    sgl_cache: &mut HashMap<String, StackGraphLanguage>,
    stack_graph: &mut stack_graphs::graph::StackGraph,
    globals: &mut tree_sitter_stack_graphs::Variables,
    language_name: &str,
    path_str : &std::path::Path,
    root_dir : &std::path::Path
) {
    let mut language_name = language_name;
    if language_name.is_empty() {
        language_name = skullian::language::name::from_file_name(path_str).unwrap_or_else(|| {
            panic!("unable to get language name from path {:?}", path_str);
        });
    }
    if sgl_cache.get(&language_name.to_string()).is_none() {
        let tsg_path = root_dir.join(skullian::language::tsg::from_language_name(&language_name).unwrap_or_else(|| {
            panic!("unable to get tsg path from language name {}", language_name);
        }));
        let grammar = skullian::language::grammar::from_language_name(&language_name).unwrap_or_else(|| {
            panic!("unable to get grammar from language name {}", language_name);
        });
        let ts_rules = std::fs::read_to_string(tsg_path).expect("stack graph rules not issued");
        sgl_cache.insert(language_name.to_string(), StackGraphLanguage::from_str(grammar, ts_rules.as_str()).unwrap());
        log::info!("StackGraph is_done_with loading sgl for {}", &language_name);
    }

    let sgl = sgl_cache.get(&language_name.to_string()).unwrap();
    stack_graph.extend(globals, path_str, sgl);
    log::info!("StackGraphProcessor is_done_with {}", path_str.display());
}

fn job_stack_graph(config: &CLIConfig) {
    let root_dir = std::env::current_dir().unwrap();
    let mut stack_graph = StackGraph::new();
    let mut globals = Variables::new();
    let mut sgl_cache = HashMap::<String, StackGraphLanguage>::new();
    for target in &config.targets {
        let mut target_path = Path::new(target);
        if !target_path.exists() {
            panic!("target {} doesn't exists", target_path.display());
        }
        if target_path.is_dir() {
            std::env::set_current_dir(target_path).unwrap_or_else(|_| {
                panic!("unable to cd to target {:?}", target_path);
            });
            target_path = Path::new(".");
        }
        for direntry in walkdir::WalkDir::new(target_path) {
            let entry = direntry.unwrap();
            let extension = entry.path().extension().unwrap_or_default().to_str().unwrap().to_string();
            if entry.file_type().is_file() {
                if
                    config.file_extension.is_empty() ||
                    (extension.len() != 0 &&
                    config.file_extension == extension)
                {
                stack_graph_process(
                    &mut sgl_cache,
                    &mut stack_graph,
                    &mut globals,
                    &config.language_name,
                    entry.path(),
                    root_dir.as_path()
                );
                }
            }
        }
        if target_path.is_dir() {
            std::env::set_current_dir(root_dir.as_path()).unwrap_or_else(|_| {
                panic!("unable to return to root dir {:?}", root_dir);
            });
        }
    }
    println!("@startuml");
    for node in stack_graph.iter_nodes() {
        let details = std::ops::Index::index(&stack_graph, node);
        let mut node_type = "enum";
        if details.is_root() {
            node_type = "interface";
        }
        if details.is_definition() {
            node_type = "class";
        }
        if details.is_reference() {
            node_type = "abstract";
        }
        
        match details.symbol() {
            Some(symbol) => {
                println!("{} n{} as \"{}\"", node_type, node.as_u32(), stack_graph.index(symbol));
            },
            None => {
                println!("{} n{}", node_type, node.as_u32());
            }
        }
    }
    for node in stack_graph.iter_nodes() {
        let edges = stack_graph.outgoing_edges(node);
        for edge in edges {
            println!("n{} ---> n{}",
                edge.source.as_u32(), edge.sink.as_u32());
        }
    }
    println!("@enduml");
}

fn job_workflow(config: &CLIConfig) {
    let mut stack_graph = StackGraph::new();
    let mut globals = Variables::new();
    let mut sgl_cache = HashMap::<String, StackGraphLanguage>::new();
    let root_dir = std::env::current_dir().unwrap();
    for target in &config.targets {
        let mut target_path = Path::new(target);
        if !target_path.exists() {
            panic!("target {} doesn't exists", target_path.display());
        }
        if target_path.is_dir() {
            std::env::set_current_dir(target_path).unwrap_or_else(|_| {
                panic!("unable to cd to target {:?}", target_path);
            });
            target_path = Path::new(".");
        }
        for direntry in walkdir::WalkDir::new(target_path) {
            let entry = direntry.unwrap();
            let extension = entry.path().extension().unwrap_or_default().to_str().unwrap().to_string();
            if entry.file_type().is_file() {
                if
                    config.file_extension.is_empty() ||
                    (extension.len() != 0 &&
                    config.file_extension == extension)
                {
                    stack_graph_process(
                        &mut sgl_cache,
                        &mut stack_graph,
                        &mut globals,
                        &config.language_name,
                        entry.path(),
                        root_dir.as_path()
                    );
                }
            }
        }
        if target_path.is_dir() {
            std::env::set_current_dir(root_dir.as_path()).unwrap_or_else(|_| {
                panic!("unable to return to root dir {:?}", root_dir);
            });
        }
    }
    let mut dep_graph = DepGraph::new();
    skullian::graph::dg::build_dep_graph(&mut dep_graph, Path::new(&config.output_file), &stack_graph);
}

fn job_debug(config: &CLIConfig) {
    let mut sgl_cache = HashMap::<String, StackGraphLanguage>::new();
    let root_dir = std::env::current_dir().unwrap();
    let mut report_text: String = String::from(
"# Quality Report

## Problems

`Problems(problem_id: TEXT, description: TEXT, causes: TEXT)`

| problem_id | description | causes |
| --------- | ----------- | ------ |
|  |  |  |

## Tests");
    for target in &config.targets {
        let target_path = Path::new(target);
        if !target_path.exists() {
            panic!("target {} doesn't exists", target_path.display());
        }
        for direntry in walkdir::WalkDir::new(target_path) {
            let entry = direntry.unwrap();
            if entry.file_type().is_file() {
                if
                    entry.path().extension().unwrap_or_default().to_str().unwrap() == "yml"
                {
                    let yaml = std::fs::read_to_string(entry.path()).unwrap();
                    let test : TestCase = serde_yaml::from_str(&yaml).unwrap();
                    std::env::set_current_dir(entry.path().parent().unwrap()).unwrap();

                    let mut dep_graph = DepGraph::new();
                    let mut stack_graph = StackGraph::new();
                    let mut globals = Variables::new();

                    for path in test.filepaths.iter() {
                        let filepath = path;
                    
                        stack_graph_process(
                            &mut sgl_cache,
                            &mut stack_graph,
                            &mut globals,
                            &config.language_name,
                            &filepath,
                            root_dir.as_path()
                        );
                    }
                    
                    skullian::graph::dg::build_dep_graph(&mut dep_graph, Path::new(""), &stack_graph);
                    match test.verify(&dep_graph) {
                        Ok(report) => {
                            report_text += format!("\n\n## {:?}\n\n{}", entry.path(), report).as_str();
                            println!("`{:?}` `ok`", entry.path());
                        },
                        Err(report) => {
                            report_text += format!("\n\n## {:?}\n\n{}", entry.path(), report).as_str();
                            println!("`{:?}` `error`", entry.path());
                        },
                    }
                    std::env::set_current_dir(root_dir.as_path()).unwrap();
                }
            }
        }
    }
    let output_file = Path::new(&config.output_file);
    if output_file.as_os_str() != "" {
        match std::fs::write(output_file, report_text) {
            Ok(_) => {},
            Err(error) => {
                panic!(
                    "unable to write report file for tests at {:?}, {}",
                    output_file, error)
            },
        }
    }
}

fn get_log_level_from_config(config: &CLIConfig) -> LevelFilter {
    match config.verbose {
        true => LevelFilter::Info,
        false => LevelFilter::Warn
    }
}

fn setup_logger(config: &CLIConfig) {
    let stdout = ConsoleAppender::builder().build();
    let logger_config = Config::builder()
        .appender(
            Appender::builder()
                .build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
            .appender("stdout")
            .build(get_log_level_from_config(config)))
        .unwrap();
    log4rs::init_config(logger_config).unwrap();
}

fn command_line() {
    let mut config = skullian::cli::CLIConfig::new_empty();
    skullian::cli::parse_args(&mut config);
    config.derive_action();
    setup_logger(&config);
    
    match config.action {
        skullian::cli::CLIAction::TreeSitter => job_tree_sitter(&config),
        skullian::cli::CLIAction::StackGraph => job_stack_graph(&config),
        skullian::cli::CLIAction::Debug => job_debug(&config),
        skullian::cli::CLIAction::Workflow => job_workflow(&config),
        _ => ()
    }
}

fn main() {
    command_line();
}
