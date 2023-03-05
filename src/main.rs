fn main() {
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
            print!("{}", skullian::graph::ts::tree_to_sexp(tree.unwrap()))
        }
    }
}