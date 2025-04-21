/*

*/

use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let input_result = File::open("data/dayXX_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut reader = BufReader::new(input);
    let mut line = String::new();
    // reader.read_line(&mut line).unwrap();
    // println!("Line: {}", line);
    // println!("Length of Line: {}", line.len());
}
