/*
Day 1: Advent of Code 2024
- select a pair of ints based on the smallest values in 2 columns
- absolute distance between a pair
- non unique entries in column allowed 
- Next iteration pick the next smallest value in the column
*/
use std::{
    fs::File, 
    io::{prelude::*, BufReader},
};

fn toi32(s: &str) -> i32 {
    /*
    Convert string to i32
    */
    s.parse::<i32>().unwrap() 
}

fn main(){ 

    let input_result = File::open("data/day1_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);

    let reader = BufReader::new(input);
    let data: Vec<String> = reader.lines()
                             .map(|result| result.unwrap())  
                             .take_while(|result| !result.is_empty())
                             .collect();

    println!("Example Data: {:?}", data[0]);    
    
    // make a vector of the first column of int type and sort
    let mut col1: Vec<i32> = data.iter().map(|x| { 
        let  v: Vec<i32> = x.split_whitespace()
                            .map(|x: &str| toi32(x))
                            .collect();
        v[0] // return first index
    }).collect();
    col1.sort();

    // make a vector of the second column of int type and sort
    let mut col2: Vec<i32> = data.iter().map(|x| { 
        let  v: Vec<i32> = x.split_whitespace()
                            .map(|x: &str| toi32(x))
                            .collect();
        v[1]
    }).collect();
    col2.sort();

    println!("Column 1: {:?}", col1.len()); 
    println!("Column 2: {:?}", col2.len());

    let _val: bool = (col1.len() == col2.len()) || panic!("Columns are not of equal length");

    // logic for problem statement
    let mut sum: i32 = 0;
    for i in 0..col1.len() { 
        sum += (col1[i] - col2[i]).abs();
    }   
    println!("Sum of absolute distances: {:?}", sum);
}