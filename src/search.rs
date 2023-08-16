use std::fs;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use super::cache::Cache;


#[allow(unused)]
pub fn from_path(path: &str) -> Cache {
    let mut path_list: Vec<String> = vec![];
    let mut file_list: Vec<String> = vec![];

    let x = match fs::read_dir(path) {
        Ok(data) => {
            for i in data {
                let item = i.unwrap().path().display().to_string();

                match fs::metadata(&item).unwrap().is_dir() {
                    true => path_list.push(item),
                    false => file_list.push(item),
                }

            }
        }
        Err(e) => println!("error: {}", e),
    };
    Cache::new(path_list, file_list)
}

#[allow(unused)]
pub fn from_path_list(path: &Vec<String>) -> Cache {
    let mut path_list: Vec<String> = vec![];
    let mut file_list: Vec<String> = vec![];

    for i in path.iter() {
        let new_list: Cache = from_path(i.as_str());
        for my_path in new_list.paths.iter() {
            path_list.push(my_path.to_owned());
        }
        for my_file in new_list.files.iter() {
            file_list.push(my_file.to_owned());
        }
    }

    Cache::new(path_list, file_list)
}

#[allow(unused)]
pub fn from_path_list2(path: &Vec<String>) -> Cache {
    let mut path_list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let mut file_list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    path
        .par_iter()
        .for_each(|f| {
            let new_list = from_path(f); 
            new_list.paths.par_iter().for_each(|x| {
                path_list.lock().unwrap().push(x.to_owned());
            });
            new_list.files.par_iter().for_each(|x| {
                file_list.lock().unwrap().push(x.to_owned());
            }); 
        });
    Cache::new(path_list.clone().lock().unwrap().to_owned(), file_list.clone().lock().unwrap().to_owned()) 
}