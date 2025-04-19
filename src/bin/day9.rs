/* 
- make more contiguous free space by compacting all of the files
- disk map: 2333133121414131402
- disk map like 12345 would represent a one-block file, two blocks of free space, a three-block file, four blocks of free space, etc
- has an ID number based on the order of the files
- Using one character for each block where digits are the file ID and . is free space
- move file blocks one at a time from the end of the disk to the leftmost free space block
- generate checksum, id*pos 
*/

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashSet, HashMap}, 
    process::exit,
}; 
use std::fmt;
use regex::Regex;

struct DisplayVec<T>(Vec<T>);

impl<T: fmt::Display> fmt::Display for DisplayVec<T> {
    // implement print method for Vec
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?; 
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

fn main(){ 
    
    let input_result = File::open("data/day9_a.txt");
    let input = match input_result { Ok(file) => file, Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut reader = BufReader::new(input);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap(); 
    println!("Line: {}", line);
    println!("Length of Line: {}", line.len());
    let mut encoded: Vec<i64> = Vec::new(); 
    let mut id: i64 = 0;
    let mut count_free = 0;
    let free_space_flag = -1; 
    for (p, c) in line.chars().enumerate() { 
        // check for even 
        // println!("Char: {} is at idx: {}", c, p);
        
        if p % 2 == 0 || p == 0 { 
            // if p is even, insert the id
            let mut count = c.to_string().parse::<i64>().unwrap();  
            // println!("file_block: {}", count);   
            while count > 0 {
                encoded.push(id); 
                count -= 1;
            }
        } else {
            // if p is odd, insert the .
            // handle none  
            let mut count = c.to_string().parse::<i64>().unwrap();
            // println!("free_block: {}", c);
            count_free += count as usize; // this is to count the free space for next step
            while count > 0 {
                encoded.push(free_space_flag); 
                count -= 1;
            }  
            id += 1;
        }
        
    }
    println!("Encoded: {}", DisplayVec(encoded.clone())); // pass a clone to not lose scope
    println!("Free space: {}", count_free); 
    let mut encoded_vec: Vec<i64> = encoded.clone();
    
    // point to left and right ends of the encoded_vec
    let mut left = 0;
    let mut right = encoded.len()-1; 
    while left < right { 
        let left_char = encoded[left];
        let right_char = encoded[right];
        if left_char == free_space_flag { 
            if right_char != free_space_flag {
                // move right to the left
                encoded_vec[left] = right_char;
                encoded_vec[right] = free_space_flag; 
                // println!("Encoded: {}", DisplayVec(encoded_vec.clone()));
                left += 1;
                right -=1;
            } else {
                right -= 1;  
            } 
        } else {
            left += 1; 
        } 
    }
    
    println!("Encoded: {} (after frag)", DisplayVec(encoded_vec.clone()));
    
    let mut checksum: i64 = 0; 
    for (p, id) in encoded_vec.iter().enumerate() { 
        if id != &free_space_flag{
                checksum += id  * p as i64; 
        }
    }
    println!("Checksum: {}", checksum);
    
}