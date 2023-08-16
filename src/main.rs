use std::time::Instant;

mod cache;
mod search;
mod write_bincode;

use cache::*;
use search::*;
use write_bincode::*;


fn main() {
    let start_path = "F:\\";
    let search_for = "DataPC_";
    let x = from_path(start_path);

    let mut old_cache: Cache = x.clone();
    let mut main_cache: Cache = x;
    let mut counter = 0;

    // get data
    let all_start = Instant::now();
    let start = Instant::now();
    loop {
            println!("counter: {}", counter);
        let new_cache: Cache = from_path_list2(&old_cache.paths);
        if Cache::is_empty(&new_cache) {
            break;
        }
        main_cache = Cache::combine(&new_cache, &main_cache);
        old_cache = new_cache;
        counter += 1;
    }
    let end = start.elapsed();
    println!("It took: {} seconds to go through: {}!", end.as_secs_f32(), start_path);

    // write to bincode
    let start_bin = Instant::now();
    write_bincode(&main_cache);
    let end_bin = start_bin.elapsed();
    println!("{}s for bin", end_bin.as_secs_f32());
    
    // get from bincode
    let start_search_bin = Instant::now();
    get_name(search_for);
    let end_search = start_search_bin.elapsed();
    println!("{}s for searching bin", end_search.as_secs_f32());

    // how long to finish program
    let end_all = all_start.elapsed();
    println!("It took: {} for all together!", end_all.as_secs_f32());

}