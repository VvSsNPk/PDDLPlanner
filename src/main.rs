extern crate pddl_parser;

use std::io::{BufRead, Write};
use std::path::PathBuf;
use team854::{generate_solutions, write_to_pddl_file};

pub mod map;
pub mod enums;

fn main() {
    let mut file = PathBuf::new();
    file.push("maps");
    let mut solutions  = PathBuf::new();
    solutions.push("maps_pddl_files");
    //write_to_single_file(&file,&PathBuf::new()).unwrap();
    write_to_pddl_file(&file).expect("error");
    generate_solutions(&solutions).unwrap();
}

