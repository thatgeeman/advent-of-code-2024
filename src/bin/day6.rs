/*

*/

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashSet, HashMap}, 
}; 
use regex::Regex;


fn main(){ 
    
    let input_result = File::open("data/day5_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut reader = BufReader::new(input);   
     
    for line in reader.by_ref().lines() {
        let line: String = line.unwrap(); 
        if line.is_empty() {  
            continue;
        }
    
         
    }
     
} 
 