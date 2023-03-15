use std::fmt::Debug;

pub enum CLIAction {
    TreeSitter(),
    StackGraph(),
    Debug(),
    Workflow(),
    Nothing()
}

impl Clone for CLIAction {
    fn clone(&self) -> Self {
        match self {
            Self::TreeSitter() => Self::TreeSitter(),
            Self::StackGraph() => Self::StackGraph(),
            Self::Debug() => Self::Debug(),
            Self::Workflow() => Self::Workflow(),
            Self::Nothing() => Self::Nothing(),
        }
    }
}

impl Debug for CLIAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TreeSitter() => f.debug_tuple("TreeSitter").finish(),
            Self::StackGraph() => f.debug_tuple("StackGraph").finish(),
            Self::Debug() => f.debug_tuple("Debug").finish(),
            Self::Workflow() => f.debug_tuple("Workflow").finish(),
            Self::Nothing() => f.debug_tuple("Nothing").finish(),
        }
    }
}

pub struct CLIConfig {
    pub file_extension: String,
    pub language_name: String,
    pub output_file: String,
    pub action: CLIAction,
    pub targets: Vec<String>,
    pub perform_tree_sitter: bool,
    pub perform_stack_graph: bool,
    pub perform_debug: bool,
    pub perform_workflow: bool
}

impl Debug for CLIConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("CLIConfig")
            .field("language_name", &self.language_name)
            .field("action", &self.action)
            .field("targets", &self.targets).finish()
    }
}

impl CLIConfig {
    pub fn new(
        file_extension: String,
        language_name: String,
        output_file: String,
        action: CLIAction,
        targets: Vec<String>,
        perform_tree_sitter: bool,
        perform_stack_graph: bool,
        perform_debug: bool,
        perform_workflow: bool
    ) -> CLIConfig {
        return CLIConfig {
            file_extension: file_extension,
            language_name: language_name,
            output_file: output_file,
            action: action,
            targets: targets,
            perform_tree_sitter: perform_tree_sitter,
            perform_stack_graph: perform_stack_graph,
            perform_debug: perform_debug,
            perform_workflow: perform_workflow
        }
    }

    pub fn new_empty() -> CLIConfig {
        return CLIConfig::new(
            String::from(""),
            String::from(""),
            String::from("./data.json"),
            CLIAction::Nothing(),
            [String::from(".")].to_vec(),
            false,
            false,
            false,
            false
        )
    }

    pub fn derive_action(&mut self) {
        if self.perform_tree_sitter {
            self.action = CLIAction::TreeSitter();
        } else if self.perform_stack_graph {
            self.action = CLIAction::StackGraph();
        } else if self.perform_debug {
            self.action = CLIAction::Debug();
        } else if self.perform_workflow {
            self.action = CLIAction::Workflow();
        }
    }
}

pub fn parse_args(config: &mut CLIConfig) {
    let mut argument_parser = argparse::ArgumentParser::new();

    argument_parser.set_description("Arcan DepGraph Generator with TS and SG");
    argument_parser
        .refer(&mut config.file_extension)
        .add_option(&["-f", "--extension"],
                    argparse::Parse,
                    "filter files by extension");
    argument_parser
        .refer(&mut config.language_name)
        .add_option(&["-l", "--language"],
                    argparse::Parse,
                    "issue the language of targets");
    argument_parser
        .refer(&mut config.output_file)
        .add_option(&["-o", "--output-file"],
                    argparse::Parse,
                    "issue the path of output file (default = ./data.json)");
    argument_parser
        .refer(&mut config.perform_stack_graph)
        .add_option(&["-S", "--stack-graph"],
                    argparse::StoreTrue,
                    "perform stack graph action");
    argument_parser
        .refer(&mut config.perform_debug)
        .add_option(&["-D", "--debug"],
                    argparse::StoreTrue,
                    "perform debug action");
    argument_parser
        .refer(&mut config.perform_workflow)
        .add_option(&["-W", "--workflow"],
                    argparse::StoreTrue,
                    "perform workflow action");
    argument_parser
        .refer(&mut config.perform_tree_sitter)
        .add_option(&["-T", "--tree-sitter"],
                    argparse::StoreTrue,
                    "perform tree sitter action");
    argument_parser
        .refer(&mut config.targets)
        .add_argument("targets",
                      argparse::Collect,
                      "target files or directories");
    argument_parser.parse_args_or_exit();
}