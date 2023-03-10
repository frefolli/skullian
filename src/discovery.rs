pub struct Discovery {
    dirpath: String,
    file_extension: Option<String>
}

impl Discovery {
    pub fn from_dirpath(dirpath: String) -> Discovery {
        Discovery {
            dirpath: dirpath,
            file_extension: None
        }
    }
    pub fn from_dirpath_and_file_extension(dirpath: String, file_extension: String) -> Discovery {
        Discovery {
            dirpath: dirpath,
            file_extension: Some(file_extension)
        }
    }

    pub fn set_dirpath(&mut self, dirpath: String) {
        self.dirpath = dirpath;
    }

    pub fn get_dirpath(&self) -> &String {
        &self.dirpath
    }
}

pub fn find_all_files(discovery: &Discovery) -> Vec<String> {
    let mut subquery = discovery.clone();
    
    let file_extension = discovery.get_file_extension();
    if file.extension.is_none() {
        for entry in std::fs::read_dir(discovery.get_dirpath()).unwrap() {
            let metadata = std::fs::metadata(entry.unwrap());
            if metadata.is_dir() {
                subquery.set_dirpath(metadata.path);
                find_all_files(&subquery);
            }
        }
    }
    [].to_vec()
}
