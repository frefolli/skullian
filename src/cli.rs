pub struct CLIConfig {
    pub file_names: Vec<String>,
    pub language_name: String
}

pub fn new_config(
    file_name: Vec<String>,
    language_name: String
) -> CLIConfig {
    return CLIConfig {
        file_names: file_name,
        language_name: language_name
    }
}

pub fn new_empty_config() -> CLIConfig {
    return CLIConfig {
        file_names: Vec::from([]),
        language_name: String::from("")
    }
}

pub fn parse_args(config: &mut CLIConfig) {
    let mut argument_parser = argparse::ArgumentParser::new();
    argument_parser.set_description("Arcan DepGraph Generator with TS and SG");
    argument_parser
        .refer(&mut config.language_name)
        .add_option(
            &["-l", "--language"],
            argparse::Store,
            "if issued uses this as language name and ignores file extensions");
    argument_parser
        .refer(&mut config.file_names)
        .add_argument(
            "file_names",
            argparse::Collect,
            "input file names");
    argument_parser.parse_args_or_exit();
}