use serde::{Serialize, Deserialize};

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cache {
    pub paths: Vec<String>,
    pub files: Vec<String>,
}

impl Cache {
    #[allow(unused)]
    pub fn new(path: Vec<String>, file: Vec<String>) -> Self {
        Cache {paths: path, files: file}
    }
    #[allow(unused)]
    pub fn combine(x: &Cache, y: &Cache) -> Cache {
        let mut final_path: Vec<String> = vec![];
        let mut final_file: Vec<String> = vec![];

        for i in x.paths.iter() {
            final_path.push(i.to_owned());
        }
        for i in x.files.iter() {
            final_file.push(i.to_owned());
        }
        for i in y.paths.iter() {
            final_path.push(i.to_owned());
        }
        for i in y.files.iter() {
            final_file.push(i.to_owned());
        }

        Cache::new(final_path, final_file)
    }
    #[allow(unused)]
    pub fn is_empty(cache: &Cache) -> bool {
        if cache.files.is_empty() && cache.paths.is_empty() {
            return true;
        }
        false
    }
}