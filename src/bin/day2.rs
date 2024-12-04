/*
Day 2: Advent of Code 2024
- read line by line (report=row, level=column)
- take absolute difference/grad along level -> all should increase or decrease
- adjacent levels differ by at least one and at most three

7 6 4 2 1
becomes 
7 6 4 2
6 4 2 1 

7 6 4 2 
becomes
7 6 4 
6 4 2
- count number of safe reports
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

fn step_difference(v: Vec<i32>) -> bool {
    // assuming length greater than 1 for each report 
    let slice1 = v[0..v.len()-1].iter(); // Right exclusive range literal 
    let slice2 = v[1..].iter();  // Right inclusive =v.len() did not work 
    let diff: Vec<i32> = slice1.zip(slice2)
                                   .map(|(x, y)| x - y)
                                   .collect();
    // println!("Difference: {:?}", diff);
    // all elements in array be increasing or decreasing
    let safe_grad_inc: bool = diff.iter().all(|&x| x >= 0);  
    let safe_grad_dec: bool = diff.iter().all(|&x| x <= 0);
    let safe_grad: bool = safe_grad_inc || safe_grad_dec;
    let safe_dist: bool = diff.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3); // adjacent levels differ by at least one and at most three
    println!("Safe Gradient: {:?}", safe_grad);
    println!("Safe Distance: {:?}", safe_dist);
    safe_grad && safe_dist
}

fn main(){ 

    let input_result = File::open("data/day2_a.txt");
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
    let mut sum: i32 = 0; 
    for line in data.iter() {
        let v: Vec<i32> = line.split_whitespace()
                            .map(|x: &str| toi32(x))
                            .collect();
        let safe:bool = step_difference(v.clone()); // clone has runtime cost, acoided by rusticians
        sum += safe as i32;
        println!("Data: {:?} -> {:?}", v, safe); 
    }
    println!("Number of safe reports: {:?}", sum);
     
}