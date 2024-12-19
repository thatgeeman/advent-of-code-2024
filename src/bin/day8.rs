/* 

*/

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashSet, HashMap}, 
}; 
use regex::Regex;


fn main(){ 
    
    let input_result = File::open("data/day8_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut reader = BufReader::new(input);   
    
}