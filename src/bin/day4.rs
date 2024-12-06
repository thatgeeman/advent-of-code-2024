/*
- find XMAS 
- can appear horizontal, vertical, diagonal, written backwards, or even overlapping other words
- horizontal and backwards: regex for XMAS and SAMX
- diagonal: iterate column wise and row wise with an offset (can be used for vertical and horizontal as well)
- vertical and backwards: iterate column wise 
- overlapping: for example: XMASAMX counts as 2 XMAS
*/ 
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn main(){  
    let input_result = File::open("data/day4_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    
    let reader = BufReader::new(input);
    let re = Regex::new(r"XMAS").unwrap();
    let rer = Regex::new(r"SAMX").unwrap();
    
    let window: usize = 4;
    // re.find_iter()
    let mut count: i32 = 0; 
    // a multidimensional vector
    let mut a: Vec<Vec<char>> = Vec::new(); 
    for line in reader.lines(){ 
        a.push(line.unwrap().chars().collect());
    } 
    println!("Text: {:?}", a);
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut max_j=0;
    let mut offset_j: usize = 0; 
    let mut offset_i: usize = 0;
 
    for i in 0..a.len() {
            for offset_j in 0..a[i].len(){ 
                let mut s: String = String::new();
                max_j = a[i].len() - window;
                if max_j < offset_j {
                    break;
                }
                // look for substrings of length window
                for j in offset_j..(window + offset_j) {
                    s.push(a[i][j]);
                }
                if re.is_match(&s) {
                    println!("Matched: {:?} at i: {:?} j: {:?} offset_j: {:?}", s, i, j, offset_j);
                    count += 1;
                }
                if rer.is_match(&s) {
                    println!("Matched: {:?} at i: {:?} j: {:?} offset_j: {:?}", s, i, j, offset_j);
                    count += 1;
                }
            }
    } 
    println!("incl. Horizontal Count: {:?}", count); 

}



