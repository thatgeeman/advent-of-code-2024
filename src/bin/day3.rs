/*
- jumbled code as input/text with mul instruction in there
- of the format mul(X, Y), X and Y are 1 to 3 digit numbers
- should contain mul() with valid X and Y to be considered valid instruction
- add up all mul(X, Y) outputs
*/
use std::{
    fs::File, 
    io::{prelude::*, BufReader},
};

use regex::Regex;

fn toi32(s: &str) -> i32 {
    /*
    Convert string to i32
    */
    s.parse::<i32>().unwrap() 
}

fn mul_str(s: &str) -> i32 {
    /*
    Get the two numbers in string and multiply them 
    */
    let re = Regex::new(r"(\d{1,3})").unwrap(); // find the groups of digits
    let mut prod: i32 = 1;
    re.find_iter(s).for_each(|x| prod *= toi32(x.as_str()));
    prod
}

fn main(){  
    let input_result = File::open("data/day3_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    
    let reader = BufReader::new(input);
    //let reader = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum: i32 = 0;
    for line in reader.lines() {  
        let line = line.unwrap();
        println!("Line: {:?}", line);
        for instr in re.find_iter(&line) { 
            sum += mul_str(instr.as_str());
        }
        println!("Sum so far: {:?}", sum);
    }
}