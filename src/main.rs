fn main() {
    let mut config = skullian::cli::new_empty_config();
    skullian::cli::parse_args(&mut config);
    println!("file_name: {:?}", config.file_names);
    println!("language_name: {}", config.language_name);
}