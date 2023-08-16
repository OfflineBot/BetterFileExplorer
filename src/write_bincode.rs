use std::{fs, io::{BufWriter, Write, BufReader}};
use bincode::{serialize, deserialize_from};

use super::cache::Cache;


#[allow(unused)]
pub fn write_bincode(data: &Cache) {
    let path = "./storage/cache.bin";
    let encoded = serialize(&data).expect("Failed serializing data");
    let file = fs::File::create(&path).expect("Failed creating File!");
    let mut write = BufWriter::new(file);
    write.write_all(&encoded).expect("Failed writing data!");
}

#[allow(unused)]
pub fn get_name(name: &str) {
    let path = "./storage/cache.bin";
    let file = fs::File::open(&path).expect("Failed opening File!");
    let reader = BufReader::new(file);

    let decoded_data: Cache = deserialize_from(reader).unwrap();
    for i in decoded_data.paths.iter() {
        if i.contains(name) {
            println!("found path: {}", i);
        }
    }
    for i in decoded_data.files.iter() {
        if i.contains(name) {
            println!("found file: {}", i);
        }
    }
}