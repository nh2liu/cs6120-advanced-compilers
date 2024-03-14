mod cfg;
mod parsing;
mod types;

use cfg::CFG;
use parsing::{parse_bril, to_json};
use std::{
    fs::read_to_string,
    io::{self, Read},
};

fn main() {
    let file_path_option = std::env::args().nth(1);
    let bril_file = file_path_option.map_or_else(
        || {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        },
        |file_path| {
            read_to_string(&file_path).expect(&format!("Could not find file path {}", file_path))
        },
    );

    let functions = parse_bril(&bril_file);
    let functions_unwrapped = functions.unwrap();

    functions_unwrapped.iter().for_each(|x| {
        let cfg = CFG::from_function(x);
        eprintln!("{}", cfg);
    });

    println!(
        "{}",
        serde_json::to_string(&to_json(&functions_unwrapped)).unwrap()
    );
}
