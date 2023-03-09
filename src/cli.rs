pub struct CLIConfig {
    pub file_name: String,
    pub language_name: String,
    pub tsg_path: String,
    pub perform_job_ts: bool,
    pub perform_job_sg: bool
}

pub fn new_config(
    file_name: String,
    language_name: String,
    tsg_path: String,
    perform_job_ts: bool,
    perform_job_sg: bool
) -> CLIConfig {
    return CLIConfig {
        file_name: file_name,
        language_name: language_name,
        tsg_path: tsg_path,
        perform_job_ts: perform_job_ts,
        perform_job_sg: perform_job_sg,
    }
}

pub fn new_empty_config() -> CLIConfig {
    return new_config(
        String::from(""),
        String::from(""),
        String::from(""),
        true, true
    )
}

pub fn parse_args(config: &mut CLIConfig) {
    let mut argument_parser = argparse::ArgumentParser::new();
    argument_parser.set_description("Arcan DepGraph Generator with TS and SG");
    argument_parser
        .refer(&mut config.language_name)
        .add_option(
            &["-l", "--language"],
            argparse::Store,
            "if issued uses this as language name and ignores file extension");
    argument_parser
        .refer(&mut config.tsg_path)
        .add_option(
            &["-s", "--tsg_path"],
            argparse::Store,
            "stack graph rules file");
        argument_parser
        .refer(&mut config.perform_job_ts)
        .add_option(
            &["-T", "--no-tree-sitter"],
            argparse::StoreFalse,
            "if issued don't perform tree sitter job");
    argument_parser
        .refer(&mut config.perform_job_sg)
        .add_option(
            &["-S", "--no-stack-graph"],
            argparse::StoreFalse,
            "if issued don't perform stack graph job");
    argument_parser
        .refer(&mut config.file_name)
        .add_argument(
            "file_name",
            argparse::Store,
            "input file");
    argument_parser.parse_args_or_exit();
}